use std::borrow::Borrow;
use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap};
use std::hash::BuildHasherDefault;
use std::ops::Index;

use crate::Color;
use crate::fractal::fractal::{Colorizer, FractalRepresentation};
use crate::interpolation::interpolatable::{InterpolatableLocation, Interpolation};
use crate::interpolation::interpolation_list::InterpolationList;
use crate::interpolation::interpolation_list::simple_hash::SimpleHasher;

const PRECISION_FACTOR: f64 = 100f64;

#[derive(Copy, Clone)]
pub struct KeyColor {
    position: f64,
    color: Color,
}

impl KeyColor {
    pub fn new(position: f64, color: Color) -> Self {
        Self {
            position,
            color,
        }
    }
}

pub trait ColorPalette {
    fn set_max(&mut self, max: f64) {}
    fn prepare(&mut self, fractal: &FractalRepresentation<impl Colorizer>) {}
    fn get_color(&mut self, index: f64) -> &Color;
}

pub struct RepeatingColorPalette {
    key_color_list: InterpolationList<Color>,
}

pub struct ScalingColorPalette {
    key_color_list: InterpolationList<Color>,
    scale: f64,
}

pub struct LogarithmicColorPalette {
    key_color_list: InterpolationList<Color>,
    pub base: f64,
}

pub struct ExponentialColorPalette {
    key_color_list: InterpolationList<Color>,
    pub exponent: f64,
    max_iter: f64,
}

pub struct HistogramColorPalette {
    key_color_list: InterpolationList<Color>,
    max_iter: f64,
    total: f64,
    cumulative_iterations: Vec<u64>,
    delta_iterations: Vec<u64>,
}

impl RepeatingColorPalette {
    pub fn new(interpolation: Interpolation, key_colors: Vec<KeyColor>) -> Self {
        Self {
            key_color_list: get_list_from_key_color(key_colors, interpolation)
        }
    }
}

impl ColorPalette for RepeatingColorPalette {
    fn get_color(&mut self, index: f64) -> &Color {
        let index = (index * PRECISION_FACTOR % (self.key_color_list.get_max_position() + 1) as f64) as u64;
        self.key_color_list.get_buffered(index)
    }
}

impl ScalingColorPalette {
    pub fn new(interpolation: Interpolation, key_colors: Vec<KeyColor>) -> Self {
        Self {
            key_color_list: get_list_from_key_color(key_colors, interpolation),
            scale: 1f64,
        }
    }
}

impl ColorPalette for ScalingColorPalette {
    fn set_max(&mut self, max: f64) {
        self.scale = max;
    }

    fn get_color(&mut self, index: f64) -> &Color {
        let index = (index * 100f64 * PRECISION_FACTOR / self.scale) as u64;
        self.key_color_list.get_buffered(index)
    }
}

impl LogarithmicColorPalette {
    pub fn new(interpolation: Interpolation, key_colors: Vec<KeyColor>, base: f64) -> Self {
        Self {
            key_color_list: get_list_from_key_color(key_colors, interpolation),
            base,
        }
    }
}

impl ColorPalette for LogarithmicColorPalette {
    fn get_color(&mut self, index: f64) -> &Color {
        let n = self.key_color_list.get_max_position() as f64;
        let factor = ((index * PRECISION_FACTOR) / n + 1.0).log(self.base) % 1f64;
        let pos = factor * n;
        self.key_color_list.get_buffered(pos as u64)
    }
}

impl ExponentialColorPalette {
    pub fn new(interpolation: Interpolation, key_colors: Vec<KeyColor>, exponent: f64) -> Self {
        Self {
            key_color_list: get_list_from_key_color(key_colors, interpolation),
            exponent,
            max_iter: 0.0,
        }
    }
}

impl ColorPalette for ExponentialColorPalette {
    fn set_max(&mut self, max: f64) {
        self.max_iter = max;
    }

    fn get_color(&mut self, index: f64) -> &Color {
        let n = self.key_color_list.get_max_position() as f64;
        let pos = ((index / self.max_iter).powf(self.exponent) * n).powf(1.5) % (n + 1.);
        self.key_color_list.get_buffered(pos as u64)
    }
}

impl HistogramColorPalette {
    pub fn new(interpolation: Interpolation, key_colors: Vec<KeyColor>) -> Self {
        Self {
            key_color_list: get_list_from_key_color(key_colors, interpolation),
            max_iter: 0.0,
            total: 0.0,
            cumulative_iterations: Vec::new(),
            delta_iterations: Vec::new(),
        }
    }
}

impl ColorPalette for HistogramColorPalette {
    fn set_max(&mut self, max: f64) {
        self.max_iter = max;
    }

    fn prepare(&mut self, fractal: &FractalRepresentation<impl Colorizer>) {
        let mut iterations_map = HashMap::<u64, u64, BuildHasherDefault<SimpleHasher>>::with_hasher(
            BuildHasherDefault::<SimpleHasher>::default()
        );

        for point in fractal.iteration_iter() {
            *iterations_map.entry(point as u64).or_default() += 1;
        }

        self.cumulative_iterations.clear();
        self.delta_iterations.clear();
        let mut total: u64 = 0;
        for i in 0..self.max_iter as u64 {
            let count = *iterations_map.entry(i).or_default();
            self.cumulative_iterations.push(total);
            self.delta_iterations.push(count);
            total += count;
        }

        self.total = total as f64;
    }

    fn get_color(&mut self, iterations: f64) -> &Color {
        let index = iterations as usize;
        let factor = (
            self.cumulative_iterations[index] as f64 +
                (iterations % 1.) * self.delta_iterations[index] as f64
        ) / self.total * PRECISION_FACTOR;
        self.key_color_list.get_buffered((factor * 100.) as u64)
    }
}


struct KeyIntColor {
    position: u64,
    color: Color,
}

fn get_list_from_key_color(key_colors: Vec<KeyColor>, interpolation: Interpolation) -> InterpolationList<Color> {
    let mut mutable_key_colors: Vec<KeyIntColor> = key_colors.iter()
        .map(|x| KeyIntColor {
            position: (x.position * PRECISION_FACTOR) as u64,
            color: x.color,
        }).collect();
    mutable_key_colors.sort_by_key(|x| x.position);
    let mut iterator = mutable_key_colors.iter();
    let mut prev = iterator.next().unwrap();

    let mut key_list = InterpolationList::new(prev.position, prev.color);

    for key_color in iterator {
        key_list.insert(interpolation, key_color.position - prev.position, key_color.color);
        prev = key_color;
    }

    key_list
}


#[cfg(test)]
mod test {
    use crate::{Color, ColorPalette, Interpolation, KeyColor, RepeatingColorPalette};
    use crate::color::color_palette::ScalingColorPalette;

    #[test]
    fn basics_repeating() {
        let key_colors = vec![
            KeyColor::new(0f64, Color::RED),
            KeyColor::new(20f64, Color::BLUE),
            KeyColor::new(10f64, Color::GREEN),
        ];

        let mut color_palette = RepeatingColorPalette::new(Interpolation::CUBIC, key_colors);

        assert_eq!(*color_palette.get_color(-0.02f64), Color::RED);
        assert_eq!(color_palette.get_color(5f64).blue, 0f32);
        assert_eq!(*color_palette.get_color(10f64), Color::GREEN);
        assert_eq!(*color_palette.get_color(20.01f64), Color::RED);
        assert_eq!(*color_palette.get_color(20f64), Color::BLUE);


        // Check outside range
        let key_colors = vec![
            KeyColor::new(5f64, Color::RED),
            KeyColor::new(20f64, Color::BLUE),
            KeyColor::new(10f64, Color::GREEN),
        ];

        let mut color_palette = RepeatingColorPalette::new(Interpolation::CUBIC, key_colors);

        assert_eq!(*color_palette.get_color(3f64), Color::RED);
    }

    #[test]
    fn basics_scaling() {
        let key_colors = vec![
            KeyColor::new(0f64, Color::RED),
            KeyColor::new(50f64, Color::GREEN),
            KeyColor::new(100f64, Color::BLUE),
        ];

        let mut color_palette = ScalingColorPalette::new(Interpolation::LINEAR, key_colors);
        color_palette.set_max(200f64);

        assert_eq!(*color_palette.get_color(100f64), Color::GREEN);
        assert_eq!(color_palette.get_color(150f64).red, 0f32);
        assert_eq!(*color_palette.get_color(250f64), Color::BLUE);
        assert_eq!(*color_palette.get_color(-100f64), Color::RED);

        color_palette.set_max(300f64);

        assert_eq!(*color_palette.get_color(150f64), Color::GREEN);
        assert_eq!(color_palette.get_color(175f64).red, 0f32);
        assert_eq!(*color_palette.get_color(350f64), Color::BLUE);
        assert_eq!(*color_palette.get_color(-15f64), Color::RED);


        // check out of range
        let key_colors = vec![
            KeyColor::new(20f64, Color::RED),
            KeyColor::new(50f64, Color::GREEN),
            KeyColor::new(100f64, Color::BLUE),
        ];

        let mut color_palette = ScalingColorPalette::new(Interpolation::LINEAR, key_colors);
        color_palette.set_max(200f64);

        assert_eq!(*color_palette.get_color(10f64), Color::RED);
        assert_eq!(*color_palette.get_color(40f64), Color::RED);
        assert!(color_palette.get_color(40.1f64).red < 1f32);
    }
}
