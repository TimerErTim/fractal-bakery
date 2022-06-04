use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};

use decimal::d128;

use crate::interpolation::interpolatable::{Interpolatable, Interpolator};

#[derive(Copy, Clone)]
pub struct Complex<T> {
    pub re: T,
    pub im: T,
}

pub type ComplexF32 = Complex<f32>;
pub type ComplexF64 = Complex<f64>;
pub type ComplexF128 = Complex<d128>;

impl<T> Complex<T> {
    #[inline]
    pub const fn new(re: T, im: T) -> Self {
        Complex { re, im }
    }
}

impl<T: Default> Complex<T> {
    pub fn default() -> Self {
        Complex::new(T::default(), T::default())
    }
}

impl<T: Add> Add<Complex<T>> for Complex<T> {
    type Output = Complex<T::Output>;

    fn add(self, rhs: Complex<T>) -> Self::Output {
        Complex::new(self.re + rhs.re, self.im + rhs.im)
    }
}

impl<T: Sub> Sub<Complex<T>> for Complex<T> {
    type Output = Complex<T::Output>;

    fn sub(self, rhs: Complex<T>) -> Self::Output {
        Complex::new(self.re - rhs.re, self.im - rhs.im)
    }
}

impl<T: Mul<T, Output=C> + Copy, C: Add<C, Output=O> + Sub<C, Output=O>, O> Mul<Complex<T>>
for Complex<T> {
    type Output = Complex<O>;

    fn mul(self, rhs: Complex<T>) -> Self::Output {
        let re = self.re * rhs.re - self.im * rhs.im;
        let im = self.re * rhs.im + self.im * rhs.re;
        Self::Output::new(re, im)
    }
}

impl<O, T: Add<O, Output=T> + Copy> AddAssign<Complex<O>> for Complex<T> {
    fn add_assign(&mut self, rhs: Complex<O>) {
        self.re = self.re + rhs.re;
        self.im = self.im + rhs.im;
    }
}

impl<O, T: Sub<O, Output=T> + Copy> SubAssign<Complex<O>> for Complex<T> {
    fn sub_assign(&mut self, rhs: Complex<O>) {
        self.re = self.re - rhs.re;
        self.im = self.im - rhs.im;
    }
}


impl<T: Interpolatable<T>> Interpolatable<Complex<T>> for Complex<T> {
    type Output = Complex<T::Output>;

    fn interpolate(&self, interpolator: &Interpolator, other: &Complex<T>) -> Self::Output {
        let new_re = interpolator.interpolate(&self.re, &other.re);
        let new_im = interpolator.interpolate(&self.im, &other.im);
        Complex::new(new_re, new_im)
    }
}