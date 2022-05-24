use crate::color_palette::ColorPalette;
use crate::rendering_settings::RenderingSettings;

pub trait Configuration {}

pub trait Fractal<C: Configuration> {
    fn configuration(&self) -> C;

    fn calculate(&self, settings: &RenderingSettings) -> dyn FractalRepresentation<C>;
}

pub trait FractalRepresentation<C: Configuration> {
    fn configuration(&self) -> C;
    fn rendering_settings(&self) -> RenderingSettings;

    fn colorize(&self, palette: &impl ColorPalette) -> image::Rgb32FImage;
}