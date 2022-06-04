use crate::{Color, ComplexF64, Mandelbrot};
use crate::fractal::fractal_animation::{AnimationIterator, FractalAnimation, Frame};
use crate::interpolation::interpolation_list::InterpolationList;

pub struct MandelbrotAnimation {
    frames: u64,
    center: InterpolationList<ComplexF64>,
    zoom_exponent: InterpolationList<f64>,
    max_iterations: InterpolationList<u128>,
    smoothing: InterpolationList<bool>,
    set_color: InterpolationList<Color>,
}

impl MandelbrotAnimation {
    pub fn new(
        frames: u64,
        center: InterpolationList<ComplexF64>,
        zoom_exponent: InterpolationList<f64>,
        max_iterations: InterpolationList<u128>,
        smoothing: InterpolationList<bool>,
        set_color: InterpolationList<Color>,
    ) -> Self {
        Self {
            frames,
            center,
            zoom_exponent,
            max_iterations,
            smoothing,
            set_color,
        }
    }
}

impl IntoIterator for MandelbrotAnimation {
    type Item = Frame<Mandelbrot>;
    type IntoIter = AnimationIterator<Mandelbrot>;

    fn into_iter(self) -> Self::IntoIter {
        AnimationIterator::new(self)
    }
}

impl FractalAnimation<Mandelbrot> for MandelbrotAnimation {
    fn get_configuration(&mut self, index: u64) -> Option<Mandelbrot> {
        if index > self.frames {
            None
        } else {
            let center = self.center.get_buffered(index);
            let zoom_exponent = self.zoom_exponent.get_buffered(index);
            let max_iterations = self.max_iterations.get_buffered(index);
            let smoothing = self.smoothing.get_buffered(index);
            let set_color = self.set_color.get_buffered(index);
            Some(Mandelbrot::new(
                *center,
                *zoom_exponent,
                *max_iterations,
                *smoothing,
                *set_color,
            ))
        }
    }
}