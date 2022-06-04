use image::{Rgb32FImage, RgbImage};

use crate::{ColorPalette, RenderingSettings};
use crate::Color;

pub trait Configuration {
    fn to_image(self, settings: &RenderingSettings, palette: &mut impl ColorPalette) -> RgbImage;
}

pub trait Fractal<T: Colorizer> {
    fn calculate(&self, settings: &RenderingSettings) -> FractalRepresentation<T>;
}

pub struct FractalRepresentation<T: Colorizer> {
    colorizer: T,
    rendering_settings: RenderingSettings,
    iteration_map: Box<[Box<[FractalPoint]>]>,
}

impl<T: Colorizer> FractalRepresentation<T> {
    pub fn new(colorizer: T, settings: RenderingSettings) -> Self {
        let iteration_map_vec = {
            let resolution = &settings.resolution;
            (0..resolution.width).map(|_|
                (0..resolution.height).map(|_|
                    FractalPoint::new()
                ).collect::<Vec<_>>().into_boxed_slice()
            ).collect::<Vec<_>>()
        };
        let iteration_map = iteration_map_vec.into_boxed_slice();

        Self {
            colorizer,
            rendering_settings: settings,
            iteration_map,
        }
    }
}

impl<T: Colorizer> FractalRepresentation<T> {
    fn rendering_settings(&self) -> &RenderingSettings {
        &self.rendering_settings
    }

    pub fn colorize(&self, palette: &mut impl ColorPalette) -> RgbImage {
        self.colorizer.setup_palette(palette);


        let resolution = &self.rendering_settings.resolution;
        let mut image = RgbImage::new(resolution.width, resolution.height);

        for x in 0..resolution.width {
            for y in 0..resolution.height {
                let color = self.iteration_map[x as usize][y as usize]
                    .get_color(&self.colorizer, palette);
                image.put_pixel(x, y, color.to_rgb());
            }
        }

        image
    }

    pub fn get_point_mut(&mut self, x: usize, y: usize) -> &mut FractalPoint {
        &mut self.iteration_map[x][y]
    }
}

pub trait Colorizer {
    fn setup_palette(&self, color_palette: &mut impl ColorPalette);

    fn get_color(&self, iterations: f64, color_palette: &mut impl ColorPalette) -> Color {
        color_palette.get_color(iterations).clone()
    }
}

pub struct FractalPoint {
    iterations: Vec<f64>,
}

impl FractalPoint {
    fn new() -> Self {
        Self {
            iterations: Vec::new()
        }
    }

    pub fn add_iteration(&mut self, iterations: f64) {
        self.iterations.push(iterations)
    }
}

impl FractalPoint {
    #[inline]
    fn get_color(&self, colorizer: &impl Colorizer, color_palette: &mut impl ColorPalette) -> Color {
        let mut colors = self.iterations.iter()
            .map(move |x| colorizer.get_color(*x, color_palette));

        Color::average_iterator(&mut colors)
    }
}


#[cfg(test)]
mod tests {

}