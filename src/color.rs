use interpolatable::Interpolatable;
use interpolatable::Interpolator;

use crate::interpolatable;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Color {
    /// Red component: 0.0 - 1.0
    pub(crate) red: f32,
    /// Green component: 0.0 - 1.0
    pub(crate) green: f32,
    /// Blue component: 0.0 - 1.0
    pub(crate) blue: f32,
}

impl Color {
    pub fn default() -> Color {
        Color::WHITE
    }
}

impl Color {
    pub fn new(red: f32, green: f32, blue: f32) -> Color {
        let red = red.max(0.0).min(1.0);
        let green = green.max(0.0).min(1.0);
        let blue = blue.max(0.0).min(1.0);
        Color { red, green, blue }
    }

    pub const BLACK: Color = Color {
        red: 0f32,
        green: 0f32,
        blue: 0f32,
    };

    pub const WHITE: Color = Color {
        red: 1f32,
        green: 1f32,
        blue: 1f32,
    };

    pub const RED: Color = Color {
        red: 1f32,
        green: 0f32,
        blue: 0f32,
    };

    pub const GREEN: Color = Color {
        red: 0f32,
        green: 1f32,
        blue: 0f32,
    };

    pub const BLUE: Color = Color {
        red: 0f32,
        green: 0f32,
        blue: 1f32,
    };

    pub const CYAN: Color = Color {
        red: 0f32,
        green: 1f32,
        blue: 1f32,
    };

    pub const MAGENTA: Color = Color {
        red: 1f32,
        green: 0f32,
        blue: 1f32,
    };

    pub const YELLOW: Color = Color {
        red: 1f32,
        green: 1f32,
        blue: 0f32,
    };
}

impl Color {
    pub fn mix(&self, other: &Color) -> Color {
        Color::new(
            (self.red + other.red) / 2f32,
            (self.green + other.green) / 2f32,
            (self.blue + other.blue) / 2f32,
        )
    }

    pub fn average(colors: &Vec<Color>) -> Color {
        let len = colors.len() as f32;

        let mut red = 0f32;
        let mut green = 0f32;
        let mut blue = 0f32;

        for color in colors {
            red += color.red;
            green += color.green;
            blue += color.blue;
        }

        Color {
            red: red / len,
            green: green / len,
            blue: blue / len,
        }
    }

    pub fn average_iterator(colors_iter: &mut impl Iterator<Item=Color>) -> Color {
        let mut amount = 0;

        let mut red = 0f32;
        let mut green = 0f32;
        let mut blue = 0f32;

        for color in colors_iter {
            red += color.red;
            green += color.green;
            blue += color.blue;
            amount += 1;
        }

        Color {
            red: red / amount as f32,
            green: green / amount as f32,
            blue: blue / amount as f32,
        }
    }
}

impl Interpolatable<Color> for Color {
    type Output = Color;

    fn interpolate(&self, interpolator: &Interpolator, other: &Color) -> Self::Output {
        let red = interpolator.interpolate(&self.red, &other.red);
        let green = interpolator.interpolate(&self.green, &other.green);
        let blue = interpolator.interpolate(&self.blue, &other.blue);
        Color { red, green, blue }
    }
}

