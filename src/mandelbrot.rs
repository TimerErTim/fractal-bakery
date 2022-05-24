use image::Rgb32FImage;

use crate::color_palette::ColorPalette;
use crate::fractal::{Configuration, Fractal, FractalRepresentation};
use crate::rendering_settings::RenderingSettings;

struct MandelbrotConfiguration {}

impl Configuration for MandelbrotConfiguration {}

struct MandelbrotView {
    configuration: MandelbrotConfiguration,
}

impl Fractal<MandelbrotConfiguration> for MandelbrotView {
    fn configuration(&self) -> MandelbrotConfiguration {
        *self.configuration
    }

    fn calculate(&self, settings: &RenderingSettings) -> MandelbrotRepresentation {
        todo!()
    }
}

struct MandelbrotRepresentation {}

impl FractalRepresentation<MandelbrotConfiguration> for MandelbrotRepresentation {
    fn configuration(&self) -> MandelbrotConfiguration {
        todo!()
    }

    fn rendering_settings(&self) -> RenderingSettings {
        todo!()
    }

    fn colorize(&self, palette: &impl ColorPalette) -> Rgb32FImage {
        todo!()
    }
}

