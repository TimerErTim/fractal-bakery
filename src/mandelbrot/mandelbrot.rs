use std::f64::consts::LN_2;

use decimal::d128;
use image::{Rgb32FImage, RgbImage};

use crate::Color;
use crate::color::color_palette::ColorPalette;
use crate::complex::complex::{Complex, ComplexF64};
use crate::fractal::fractal::{Colorizer, Configuration, Fractal, FractalRepresentation};
use crate::RenderingSettings;

#[derive(Clone)]
pub struct Mandelbrot {
    center: ComplexF64,
    zoom_exponent: f64,
    max_iterations: u128,
    smoothing: bool,
    set_color: Color,
}

impl Mandelbrot {
    pub fn new(
        center: ComplexF64,
        zoom_exponent: f64,
        max_iterations: u128,
        smoothing: bool,
        set_color: Color,
    ) -> Self {
        Self {
            center,
            zoom_exponent,
            max_iterations,
            smoothing,
            set_color,
        }
    }
}

impl Configuration for Mandelbrot {
    fn to_image(self, settings: &RenderingSettings, palette: &mut impl ColorPalette) -> RgbImage {
        let representation = self.calculate(settings);
        representation.colorize(palette)
    }
}

impl Colorizer for Mandelbrot {
    fn setup_palette(&self, color_palette: &mut impl ColorPalette) {
        color_palette.set_max(self.max_iterations as f64)
    }

    fn get_color(&self, iterations: f64, color_palette: &mut impl ColorPalette) -> Color {
        if iterations == self.max_iterations as f64 {
            self.set_color.clone()
        } else {
            color_palette.get_color(iterations).clone()
        }
    }
}

impl Fractal<Mandelbrot> for Mandelbrot {
    fn calculate(&self, settings: &RenderingSettings) -> FractalRepresentation<Mandelbrot> {
        let mut representation = FractalRepresentation::new(
            self.clone(),
            settings.clone(),
        );
        let pixel_step = if settings.resolution.width as f32 * 1.5 > settings.resolution.height as f32 {
            2f64 / (self.zoom_exponent.exp() * settings.resolution.height as f64)
        } else {
            3f64 / (self.zoom_exponent.exp() * settings.resolution.width as f64)
        };
        let re_samples = settings.sampling.samples_x();
        let im_samples = settings.sampling.samples_y();
        let er: f64 = if self.smoothing { 8.0 } else { 2.0 };
        let er_square = er * er;
        let er_ln = er.ln();

        for x in 0..settings.resolution.width {
            for y in 0..settings.resolution.height {
                let point = representation.get_point_mut(x as usize, y as usize);

                for re_sample in 0..re_samples {
                    let x_pixel = x as f64 + re_sample as f64 / re_samples as f64;
                    for im_sample in 0..im_samples {
                        let y_pixel = y as f64 + im_sample as f64 / im_samples as f64;

                        let c = Complex::new(
                            pixel_step * (x_pixel - settings.resolution.width as f64 / 2.0),
                            pixel_step * (settings.resolution.height as f64 / 2.0 - y_pixel),
                        ) + self.center;

                        let mut iteration = 0u128;
                        let mut z_re = 0.;
                        let mut z_im = 0.;
                        let mut z_re_square = 0.;
                        let mut z_im_square = 0.;
                        while iteration < self.max_iterations && z_re_square + z_im_square < er_square {
                            // let mut z = ComplexF64::default();
                            // z = z * z + c;

                            z_im = (z_re + z_re) * z_im + c.im;
                            z_re = z_re_square - z_im_square + c.re;
                            z_re_square = z_re * z_re;
                            z_im_square = z_im * z_im;

                            iteration += 1;
                        }

                        let iteration = if self.smoothing && iteration < self.max_iterations {
                            //let renormalized = (z.re * z.re + z.im * z.im).sqrt().ln().ln() / LN_2;
                            //let iteration = iteration as f64 - escape_radius.ln().ln() / LN_2;
                            //iteration + 1.0 - renormalized
                            let compensation = (er_ln * (z_re_square + z_im_square).sqrt().ln()).ln() / LN_2;
                            iteration as f64 + 1.0 - compensation
                        } else {
                            iteration as f64
                        };
                        point.add_iteration(iteration);
                    }
                }
            }
        }

        representation
    }
}

