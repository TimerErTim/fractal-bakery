use std::time::{Instant, SystemTime};

use iced::futures::stream::iter;
use iced::futures::StreamExt;

use crate::color::Color;
use crate::color_palette::ColorPalette;
use crate::rendering_settings::RenderingSettings;

pub trait Configuration {}

pub trait Fractal {
    fn calculate(&self, settings: &RenderingSettings) -> Box<dyn FractalRepresentation>;
}

pub trait FractalRepresentation {
    fn rendering_settings(&self) -> RenderingSettings;

    fn colorize(&self, palette: &dyn ColorPalette) -> image::Rgb32FImage;
}

pub struct FractalCalculatedPoint {
    iterations: [f64; 4],
}

impl FractalCalculatedPoint {
    pub fn get_color(&self, color_palette: &mut impl ColorPalette) -> Color {
        let mut colors = self.iterations.iter()
            .map(|x| color_palette.get_color(*x).clone());

        Color::average_iterator(&mut colors)
    }
}


#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use crate::color::Color;
    use crate::color_palette::{ColorPalette, KeyColor, RepeatingColorPalette};
    use crate::fractal::FractalCalculatedPoint;
    use crate::interpolatable::Interpolation;
    use crate::rendering_settings::MultiSampling;

    #[test]
    fn get_sampled_color() {
        let mut color_palette = RepeatingColorPalette::new(
            Interpolation::LINEAR,
            vec![
                KeyColor::new(0f64, Color::RED),
                KeyColor::new(100f64, Color::BLUE),
                KeyColor::new(1000f64, Color::GREEN),
            ],
        );

        for i in 0..2160 * 3840 {
            let fractal_point = FractalCalculatedPoint {
                iterations: [50f64, 100f64, 200f64, 0f64]
            };

            let _ = fractal_point.get_color(&mut color_palette);
        }
    }
}