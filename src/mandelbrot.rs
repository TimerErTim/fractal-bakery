use decimal::d128;
use image::Rgb32FImage;

use crate::color::Color;
use crate::color_palette::ColorPalette;
use crate::complex::{Complex, ComplexF128};
use crate::fractal::{Configuration, Fractal, FractalRepresentation};
use crate::rendering_settings::RenderingSettings;

struct MandelbrotConfiguration {
    center: ComplexF128,
    zoom_exponent: d128,
    max_iterations: u128,
    smoothing: bool,
    set_color: Color,
}

impl Configuration for MandelbrotConfiguration {}

struct MandelbrotView {
    configuration: MandelbrotConfiguration,
}

impl Fractal for MandelbrotView {
    fn calculate(&self, settings: &RenderingSettings) -> Box<dyn FractalRepresentation> {
        todo!()
    }
}

struct MandelbrotRepresentation {}

impl FractalRepresentation for MandelbrotRepresentation {
    fn rendering_settings(&self) -> RenderingSettings {
        todo!()
    }

    fn colorize(&self, palette: &dyn ColorPalette) -> Rgb32FImage {
        todo!()
    }
}

