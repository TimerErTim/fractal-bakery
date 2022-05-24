use std::collections::BTreeMap;
use std::ops::Index;

use color::Color;

use crate::color;
use crate::interpolatable::{InterpolatableLocation, Interpolation};

pub trait ColorPalette: Index<f64> {
    fn set_max(&mut self, max: f64);
}

pub struct RepeatingColorPalette {
    pub interpolation: Interpolation,
    pub key_colors: BTreeMap<f64, Color>,
}

pub struct ScalingColorPalette {
    pub interpolation: Interpolation,
    pub key_colors: BTreeMap<f64, Color>,
    pub scale: f64,
}

impl Index<f64> for RepeatingColorPalette {
    type Output = Color;

    fn index(&self, index: f64) -> &Self::Output {
        let index = index % self.key_colors.keys().max().unwrap();
        get_color_from_keys(&self.key_colors, index, &self.interpolation)
    }
}

impl ColorPalette for RepeatingColorPalette {
    fn set_max(&mut self, _: f64) {}
}

impl Index<f64> for ScalingColorPalette {
    type Output = Color;

    fn index(&self, index: f64) -> &Self::Output {
        let index = index / self.scale;
        get_color_from_keys(&self.key_colors, index, &self.interpolation)
    }
}

impl ColorPalette for ScalingColorPalette {
    fn set_max(&mut self, max: f64) {
        self.scale = max;
    }
}

fn get_color_from_keys(
    keys: &BTreeMap<f64, Color>,
    index: f64,
    interpolation: &Interpolation,
) -> &Color {
    if let Some(mid) = keys.get(&index) {
        return mid;
    }

    let first = keys.range(..index).next_back();
    let second = keys.range(index..).next();

    if let (Some((index_first, color_first)), Some((index_second, color_second))) = (first, second) {
        let left_interpolatable = InterpolatableLocation {
            value: color_first,
            location: *index_first,
        };
        let right_interpolatable = InterpolatableLocation {
            value: color_second,
            location: *index_second,
        };
        interpolation.interpolate(left_interpolatable, index, right_interpolatable)
    } else if let Some((_, color_first)) = first {
        color_first
    } else if let Some((_, color_second)) = second {
        color_second
    } else {
        panic!("ColorPalette has no key colors")
    }
}