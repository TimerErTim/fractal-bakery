use image::Rgb;

use crate::interpolation::interpolatable::{Interpolatable, Interpolator};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Color {
    /// Red component: 0.0 - 1.0
    pub(crate) red: f32,
    /// Green component: 0.0 - 1.0
    pub(crate) green: f32,
    /// Blue component: 0.0 - 1.0
    pub(crate) blue: f32,
}

impl Default for Color {
    fn default() -> Color {
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

    pub fn average(colors: &mut Vec<Color>) -> Color {
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

impl Color {
    pub fn to_rgb(self) -> Rgb<u8> {
        Rgb([
            (self.red * u8::MAX as f32) as u8,
            (self.green * u8::MAX as f32) as u8,
            (self.blue * u8::MAX as f32) as u8
        ])
    }

    pub fn to_rgbf32(self) -> Rgb<f32> {
        Rgb([self.red, self.green, self.blue])
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


#[cfg(test)]
mod test {
    use crate::{Color, Interpolation};
    use crate::color::Color;
    use crate::interpolatable::Interpolation;

    #[test]
    fn basics() {
        let out_of_bounds_color = Color::new(-1f32, 0.6f32, 1.24);

        assert_eq!(out_of_bounds_color, Color {
            red: 0f32,
            green: 0.6f32,
            blue: 1f32,
        });

        assert_eq!(Color::default(), Color::WHITE);
    }

    #[test]
    fn mixing() {
        assert_eq!(Color::BLACK.mix(&Color::WHITE), Color {
            red: 0.5,
            green: 0.5,
            blue: 0.5,
        });

        assert_eq!(Color::BLUE.mix(&Color::WHITE), Color {
            red: 0.5,
            green: 0.5,
            blue: 1f32,
        });

        assert_eq!(Color::BLUE.mix(&Color::WHITE), Color::WHITE.mix(&Color::BLUE));

        assert_eq!(Color::RED.mix(&Color::GREEN), Color {
            red: 0.5,
            green: 0.5,
            blue: 0f32,
        });

        let red_and_green = Color::RED.mix(&Color::GREEN);
        assert_eq!(Color::YELLOW.mix(&red_and_green), Color {
            red: 0.75,
            green: 0.75,
            blue: 0f32,
        });
        assert_eq!(Color::YELLOW.mix(&red_and_green), red_and_green.mix(&Color::YELLOW));


        let mut colors = vec![Color::RED, Color::GREEN, Color::BLUE, Color::BLACK];
        assert_eq!(Color::average(&mut colors), Color {
            red: 0.25,
            green: 0.25,
            blue: 0.25,
        });
        assert_eq!(Color::average(&mut colors), Color::average_iterator(&mut colors.into_iter()));
    }

    #[test]
    fn interpolation() {
        let mixture = Interpolation::LINEAR.interpolate(&Color::RED, 0.25, &Color::GREEN);
        assert_eq!(mixture, Color {
            red: 0.75,
            green: 0.25,
            blue: 0f32,
        });

        let mixture = Interpolation::CUBIC.interpolate(&Color::RED, 0.2, &Color::GREEN);
        assert_eq!(mixture, Color {
            red: 0.896,
            green: 0.104,
            blue: 0f32,
        });
    }
}
