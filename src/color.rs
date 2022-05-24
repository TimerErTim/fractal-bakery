use interpolatable::Interpolatable;
use interpolatable::Interpolator;

use crate::interpolatable;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Color {
    /// Red component: 0.0 - 1.0
    red: f32,
    /// Green component: 0.0 - 1.0
    green: f32,
    /// Blue component: 0.0 - 1.0
    blue: f32,
}

impl Color {
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

impl Interpolatable<Color> for Color {
    type Output = Color;

    fn interpolate(&self, interpolator: &Interpolator, other: &Color) -> Self::Output {
        let red = interpolator.interpolate(&self.red, &other.red);
        let green = interpolator.interpolate(&self.green, &other.green);
        let blue = interpolator.interpolate(&self.blue, &other.blue);
        Color { red, green, blue }
    }
}

