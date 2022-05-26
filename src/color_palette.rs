use std::borrow::Borrow;
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::ops::Index;

use color::Color;

use crate::color;
use crate::interpolatable::{InterpolatableLocation, Interpolation};

pub trait ColorPalette {
    fn set_max(&mut self, max: f64);
    fn get_color(&self, index: f64) -> Color;
}

pub struct RepeatingColorPalette {
    pub interpolation: Interpolation,
    pub key_colors: BTreeMap<u128, Color>,
}

pub struct ScalingColorPalette {
    pub interpolation: Interpolation,
    pub key_colors: BTreeMap<u128, Color>,
    pub scale: f64,
}

impl ColorPalette for RepeatingColorPalette {
    fn set_max(&mut self, _: f64) {}

    fn get_color(&self, index: f64) -> Color {
        let index = (index * 100f64 % (*self.key_colors.keys().max().unwrap() + 1) as f64) as u128;
        get_color_from_keys(&self.key_colors, index, &self.interpolation)
    }
}

impl ColorPalette for ScalingColorPalette {
    fn set_max(&mut self, max: f64) {
        self.scale = max;
    }

    fn get_color(&self, index: f64) -> Color {
        let index = ((index / self.scale) * 100f64) as u128;
        get_color_from_keys(&self.key_colors, index, &self.interpolation)
    }
}

fn get_color_from_keys(
    keys: &BTreeMap<u128, Color>,
    index: u128,
    interpolation: &Interpolation,
) -> Color {
    if let Some(mid) = keys.get(&index) {
        return *mid;
    }

    let first = keys.range(..index).next_back();
    let second = keys.range(index..).next();

    if let (Some((index_first, color_first)), Some((index_second, color_second))) = (first, second) {
        let left_interpolatable = InterpolatableLocation::new(
            color_first,
            *index_first as f64,
        );
        let right_interpolatable = InterpolatableLocation::new(
            color_second,
            *index_second as f64,
        );
        interpolation.interpolate(&left_interpolatable, index as f64, &right_interpolatable)
    } else if let Some((_, color_first)) = first {
        *color_first
    } else if let Some((_, color_second)) = second {
        *color_second
    } else {
        panic!("ColorPalette has no key colors")
    }
}
