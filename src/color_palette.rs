use std::borrow::Borrow;
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::ops::Index;

use iced::futures::stream::iter;

use color::Color;

use crate::color;
use crate::interpolatable::{InterpolatableLocation, Interpolation};
use crate::interpolation_list::InterpolationList;

const PRECISION_FACTOR: f64 = 100f64;

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
    fn new(interpolation: Interpolation, key_colors: Vec<KeyColor>) -> Self where Self: Sized;
    fn set_max(&mut self, max: f64);
    fn get_color(&mut self, index: f64) -> &Color;
}

pub struct RepeatingColorPalette {
    key_color_list: InterpolationList<Color>,
}

pub struct ScalingColorPalette {
    key_color_list: InterpolationList<Color>,
    scale: f64,
}

impl ColorPalette for RepeatingColorPalette {
    fn new(interpolation: Interpolation, key_colors: Vec<KeyColor>) -> Self {
        Self {
            key_color_list: get_list_from_key_color(key_colors, interpolation)
        }
    }

    fn set_max(&mut self, _: f64) {}

    fn get_color(&mut self, index: f64) -> &Color {
        let index = (index * PRECISION_FACTOR % (self.key_color_list.get_max_position() + 1) as f64) as u64;
        self.key_color_list.get_buffered(index)
    }
}

impl ColorPalette for ScalingColorPalette {
    fn new(interpolation: Interpolation, key_colors: Vec<KeyColor>) -> Self {
        Self {
            key_color_list: get_list_from_key_color(key_colors, interpolation),
            scale: 1f64,
        }
    }

    fn set_max(&mut self, max: f64) {
        self.scale = max;
    }

    fn get_color(&mut self, index: f64) -> &Color {
        let index = (index * 100f64 * PRECISION_FACTOR / self.scale) as u64;
        self.key_color_list.get_buffered(index)
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
    use crate::color::Color;
    use crate::color_palette::{ColorPalette, KeyColor, RepeatingColorPalette, ScalingColorPalette};
    use crate::interpolatable::Interpolation;

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
