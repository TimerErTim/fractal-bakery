use std::slice::Iter;

use crate::fractal::fractal::{Colorizer, FractalPoint, FractalRepresentation};

pub struct FractalRepresentationIterator<'a, T: Colorizer> {
    fractal: &'a FractalRepresentation<T>,
    x: u32,
    y: u32,
}

impl<'a, T: Colorizer> FractalRepresentationIterator<'a, T> {
    pub fn new(fractal: &'a FractalRepresentation<T>) -> Self {
        Self {
            fractal,
            x: 0,
            y: 0,
        }
    }
}

impl<'a, T: Colorizer> Iterator for FractalRepresentationIterator<'a, T> {
    type Item = &'a FractalPoint;

    fn next(&mut self) -> Option<Self::Item> {
        if self.x < self.fractal.width() {
            let point = self.fractal.get_point(self.x as usize, self.y as usize);
            self.x += 1;
            return Some(point);
        } else {
            self.y += 1;
            self.x = 1;
            if self.y < self.fractal.height() {
                return Some(self.fractal.get_point(0, self.y as usize));
            }
        }

        return None;
    }
}

pub struct FractalIterationIterator<'a, T: Colorizer> {
    representation_iter: FractalRepresentationIterator<'a, T>,
    point_iter: Iter<'a, f64>,
}

impl<'a, T: Colorizer> FractalIterationIterator<'a, T> {
    pub fn new(fractal: &'a FractalRepresentation<T>) -> Self {
        let mut representation_iter = fractal.iter();
        let initial_point_iter = representation_iter.next().unwrap().iter();
        Self {
            representation_iter,
            point_iter: initial_point_iter,
        }
    }
}

impl<'a, T: Colorizer> Iterator for FractalIterationIterator<'a, T> {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        return if let Some(next_iteration) = self.point_iter.next() {
            Some(*next_iteration)
        } else {
            if let Some(next_point) = self.representation_iter.next() {
                self.point_iter = next_point.iter();
                self.point_iter.next().cloned()
            } else {
                None
            }
        }
    }
}
