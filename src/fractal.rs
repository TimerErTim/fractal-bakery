use std::time::{Instant, SystemTime};

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
    pub fn get_color(&self, color_palette: &impl ColorPalette) -> (Color, f64) {
        let prev_time = Instant::now();
        let mut colors = self.iterations.iter()
            .map(|x| color_palette.get_color(*x));
        let elapsed = prev_time.elapsed().as_secs_f64();


        /*while colors.len() > 1 {
            let mut new_colors = Vec::new();
            let mut colors_iterator = colors.iter();
            while let Some(x1) = colors_iterator.next() {
                if let Some(x2) = colors_iterator.next() {
                    new_colors.push(x1.mix(x2));
                }
            }
            colors = new_colors;
            new_colors = Vec::new();
        }*/

        /*let mut red = 0f32;
        let mut green = 0f32;
        let mut blue = 0f32;

        for color in &colors {
            red += color.red;
            green += color.green;
            blue += color.blue;
        }

        let color = Color::new(red / colors.len() as f32, green / colors.len() as f32, blue / colors.len() as f32);
*/

        (Color::average(&mut colors), elapsed)
    }
}


#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use crate::color::Color;
    use crate::color_palette::RepeatingColorPalette;
    use crate::fractal::FractalCalculatedPoint;
    use crate::interpolatable::Interpolation;
    use crate::rendering_settings::MultiSampling;

    #[test]
    fn get_sampled_color() {
        let mut key_colors = BTreeMap::new();
        key_colors.insert(0u128, Color::RED);
        key_colors.insert(100_00u128, Color::BLUE);
        key_colors.insert(1000_00u128, Color::GREEN);


        let color_palette = RepeatingColorPalette {
            key_colors,
            interpolation: Interpolation::LINEAR,
        };

        let mut sum = 0f64;

        for i in 0..2160 * 3840 {
            let fractal_point = FractalCalculatedPoint {
                iterations: [50f64, 100f64, 0f64, 500f64]
            };

            let (color, elapsed) = fractal_point.get_color(&color_palette);
            sum += elapsed;
        }

        println!("{sum}");
    }
}