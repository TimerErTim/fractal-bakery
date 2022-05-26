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
    iterations: [f64; 1],
}

impl FractalCalculatedPoint {
    pub fn get_color(&self, color_palette: &impl ColorPalette) -> Color {
        let mut colors: Vec<Color> = self.iterations.iter()
            .map(|x| color_palette.get_color(*x)).collect();


        while colors.len() > 1 {
            let mut new_colors = Vec::new();
            let mut colors_iterator = colors.iter();
            while let Some(x1) = colors_iterator.next() {
                if let Some(x2) = colors_iterator.next() {
                    new_colors.push(x1.mix(x2));
                }
            }
            colors = new_colors;
            new_colors = Vec::new();
        }

        colors[0]
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

        for i in 0..2160 * 3840 {
            let fractal_point = FractalCalculatedPoint {
                iterations: [50f64]
            };

            let color = fractal_point.get_color(&color_palette);
            //println!("{}: {:?}", i, color);
        }
    }
}