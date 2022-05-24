use decimal::d128;

use crate::interpolatable::Interpolation::EXPONENTIAL;

#[derive(Copy, Clone)]
pub enum Interpolation {
    LINEAR,
    CUBIC,
    NEAREST,
    EXPONENTIAL(f64),
}

impl Interpolation {
    pub fn interpolator(self) -> Interpolator {
        Interpolator::new(self)
    }

    pub fn interpolate<R, T: Interpolatable<R>>(self, first: T, ratio: f64, second: R) -> T::Output {
        let mut interpolator = self.interpolator();
        interpolator.ratio = ratio;
        interpolator.interpolate(first, second)
    }
}

pub struct Interpolator {
    interpolation: Interpolation,
    ratio: f64,
}

impl Interpolator {
    fn new(interpolation: Interpolation) -> Interpolator {
        Interpolator { interpolation, ratio: 0f64 }
    }
}

impl Interpolator {
    fn interpolate_primitive<T, R>(&self, this: &T, other: &R) -> T::Output
        where T: InterpolatablePrimitive<R> {
        match self.interpolation {
            Interpolation::LINEAR => {
                this.interpolate_linear(self.ratio, other)
            }
            Interpolation::CUBIC => {
                this.interpolate_cubic(self.ratio, other)
            }
            Interpolation::NEAREST => {
                this.interpolate_nearest(self.ratio, other)
            }
            EXPONENTIAL(n) => {
                this.interpolate_exponential(self.ratio, n, other)
            }
        }
    }

    pub(crate) fn interpolate<T, R>(&self, this: &T, other: &R) -> T::Output
        where T: Interpolatable<R> {
        this.interpolate(self, other)
    }
}

pub trait Interpolatable<R> {
    type Output;

    fn interpolate(&self, interpolator: &Interpolator, other: &R) -> Self::Output;
}

pub trait InterpolatablePrimitive<R> {
    type Output;

    fn interpolate_nearest(&self, ratio: f64, other: &R) -> Self::Output;
    fn interpolate_linear(&self, ratio: f64, other: &R) -> Self::Output;
    fn interpolate_cubic(&self, ratio: f64, other: &R) -> Self::Output;
    fn interpolate_exponential(&self, ratio: f64, exponent: f64, other: &R) -> Self::Output;
}

impl InterpolatablePrimitive<i8> for i8 {
    type Output = i8;

    fn interpolate_nearest(&self, ratio: f64, other: &i8) -> Self::Output {
        if ratio < 0.5 {
            *self
        } else {
            *other
        }
    }

    fn interpolate_linear(&self, ratio: f64, other: &i8) -> Self::Output {
        (other - self) * ratio + self
    }

    fn interpolate_cubic(&self, ratio: f64, other: &i8) -> Self::Output {
        let difference = self - other;
        2 * difference * ratio * ratio * ratio -
            3 * difference * ratio * ratio +
            self
    }

    fn interpolate_exponential(&self, ratio: f64, exponent: f64, other: &i8) -> Self::Output {
        let a = -1 + self;
        (ratio.pow(exponent) * ((other - a) as f64).ln()).exp() + a
    }
}

impl InterpolatablePrimitive<i16> for i16 {
    type Output = i16;

    fn interpolate_nearest(&self, ratio: f64, other: &i16) -> Self::Output {
        if ratio < 0.5 {
            *self
        } else {
            *other
        }
    }

    fn interpolate_linear(&self, ratio: f64, other: &i16) -> Self::Output {
        (other - self) * ratio + self
    }

    fn interpolate_cubic(&self, ratio: f64, other: &i16) -> Self::Output {
        let difference = self - other;
        2 * difference * ratio * ratio * ratio -
            3 * difference * ratio * ratio +
            self
    }

    fn interpolate_exponential(&self, ratio: f64, exponent: f64, other: &i16) -> Self::Output {
        let a = -1 + self;
        (ratio.pow(exponent) * ((other - a) as f64).ln()).exp() + a
    }
}

impl InterpolatablePrimitive<i32> for i32 {
    type Output = i32;

    fn interpolate_nearest(&self, ratio: f64, other: &i32) -> Self::Output {
        if ratio < 0.5 {
            *self
        } else {
            *other
        }
    }

    fn interpolate_linear(&self, ratio: f64, other: &i32) -> Self::Output {
        (other - self) * ratio + self
    }

    fn interpolate_cubic(&self, ratio: f64, other: &i32) -> Self::Output {
        let difference = self - other;
        2 * difference * ratio * ratio * ratio -
            3 * difference * ratio * ratio +
            self
    }

    fn interpolate_exponential(&self, ratio: f64, exponent: f64, other: &i32) -> Self::Output {
        let a = -1 + self;
        (ratio.pow(exponent) * ((other - a) as f64).ln()).exp() + a
    }
}

impl InterpolatablePrimitive<i64> for i64 {
    type Output = i64;

    fn interpolate_nearest(&self, ratio: f64, other: &i64) -> Self::Output {
        if ratio < 0.5 {
            *self
        } else {
            *other
        }
    }

    fn interpolate_linear(&self, ratio: f64, other: &i64) -> Self::Output {
        (other - self) * ratio + self
    }

    fn interpolate_cubic(&self, ratio: f64, other: &i64) -> Self::Output {
        let difference = self - other;
        2 * difference * ratio * ratio * ratio -
            3 * difference * ratio * ratio +
            self
    }

    fn interpolate_exponential(&self, ratio: f64, exponent: f64, other: &i64) -> Self::Output {
        let a = -1 + self;
        (ratio.pow(exponent) * ((other - a) as f64).ln()).exp() + a
    }
}

impl InterpolatablePrimitive<i128> for i128 {
    type Output = i128;

    fn interpolate_nearest(&self, ratio: f64, other: &i128) -> Self::Output {
        if ratio < 0.5 {
            *self
        } else {
            *other
        }
    }

    fn interpolate_linear(&self, ratio: f64, other: &i128) -> Self::Output {
        (other - self) * ratio + self
    }

    fn interpolate_cubic(&self, ratio: f64, other: &i128) -> Self::Output {
        let difference = self - other;
        2 * difference * ratio * ratio * ratio -
            3 * difference * ratio * ratio +
            self
    }

    fn interpolate_exponential(&self, ratio: f64, exponent: f64, other: &i128) -> Self::Output {
        let a = -1 + self;
        (ratio.pow(exponent) * ((other - a) as f64).ln()).exp() + a
    }
}

impl InterpolatablePrimitive<u8> for u8 {
    type Output = u8;

    fn interpolate_nearest(&self, ratio: f64, other: &u8) -> Self::Output {
        if ratio < 0.5 {
            *self
        } else {
            *other
        }
    }

    fn interpolate_linear(&self, ratio: f64, other: &u8) -> Self::Output {
        (other - self) * ratio + self
    }

    fn interpolate_cubic(&self, ratio: f64, other: &u8) -> Self::Output {
        let difference = self - other;
        2 * difference * ratio * ratio * ratio -
            3 * difference * ratio * ratio +
            self
    }

    fn interpolate_exponential(&self, ratio: f64, exponent: f64, other: &u8) -> Self::Output {
        let a = -1 + self;
        (ratio.pow(exponent) * ((other - a) as f64).ln()).exp() + a
    }
}

impl InterpolatablePrimitive<u16> for u16 {
    type Output = u16;

    fn interpolate_nearest(&self, ratio: f64, other: &u16) -> Self::Output {
        if ratio < 0.5 {
            *self
        } else {
            *other
        }
    }

    fn interpolate_linear(&self, ratio: f64, other: &u16) -> Self::Output {
        (other - self) * ratio + self
    }

    fn interpolate_cubic(&self, ratio: f64, other: &u16) -> Self::Output {
        let difference = self - other;
        2 * difference * ratio * ratio * ratio -
            3 * difference * ratio * ratio +
            self
    }

    fn interpolate_exponential(&self, ratio: f64, exponent: f64, other: &u16) -> Self::Output {
        let a = -1 + self;
        (ratio.pow(exponent) * ((other - a) as f64).ln()).exp() + a
    }
}

impl InterpolatablePrimitive<u32> for u32 {
    type Output = u32;

    fn interpolate_nearest(&self, ratio: f64, other: &u32) -> Self::Output {
        if ratio < 0.5 {
            *self
        } else {
            *other
        }
    }

    fn interpolate_linear(&self, ratio: f64, other: &u32) -> Self::Output {
        (other - self) * ratio + self
    }

    fn interpolate_cubic(&self, ratio: f64, other: &u32) -> Self::Output {
        let difference = self - other;
        2 * difference * ratio * ratio * ratio -
            3 * difference * ratio * ratio +
            self
    }

    fn interpolate_exponential(&self, ratio: f64, exponent: f64, other: &u32) -> Self::Output {
        let a = -1 + self;
        (ratio.pow(exponent) * ((other - a) as f64).ln()).exp() + a
    }
}

impl InterpolatablePrimitive<u64> for u64 {
    type Output = u64;

    fn interpolate_nearest(&self, ratio: f64, other: &u64) -> Self::Output {
        if ratio < 0.5 {
            *self
        } else {
            *other
        }
    }

    fn interpolate_linear(&self, ratio: f64, other: &u64) -> Self::Output {
        (other - self) * ratio + self
    }

    fn interpolate_cubic(&self, ratio: f64, other: &u64) -> Self::Output {
        let difference = self - other;
        2 * difference * ratio * ratio * ratio -
            3 * difference * ratio * ratio +
            self
    }

    fn interpolate_exponential(&self, ratio: f64, exponent: f64, other: &u64) -> Self::Output {
        let a = -1 + self;
        (ratio.pow(exponent) * ((other - a) as f64).ln()).exp() + a
    }
}

impl InterpolatablePrimitive<u128> for u128 {
    type Output = u128;

    fn interpolate_nearest(&self, ratio: f64, other: &u128) -> Self::Output {
        if ratio < 0.5 {
            *self
        } else {
            *other
        }
    }

    fn interpolate_linear(&self, ratio: f64, other: &u128) -> Self::Output {
        (other - self) * ratio + self
    }

    fn interpolate_cubic(&self, ratio: f64, other: &u128) -> Self::Output {
        let difference = self - other;
        2 * difference * ratio * ratio * ratio -
            3 * difference * ratio * ratio +
            self
    }

    fn interpolate_exponential(&self, ratio: f64, exponent: f64, other: &u128) -> Self::Output {
        let a = -1 + self;
        (ratio.pow(exponent) * ((other - a) as f64).ln()).exp() + a
    }
}

impl InterpolatablePrimitive<f32> for f32 {
    type Output = f32;

    fn interpolate_nearest(&self, ratio: f64, other: &f32) -> Self::Output {
        if ratio < 0.5 {
            *self
        } else {
            *other
        }
    }

    fn interpolate_linear(&self, ratio: f64, other: &f32) -> Self::Output {
        (other - self) * ratio + self
    }

    fn interpolate_cubic(&self, ratio: f64, other: &f32) -> Self::Output {
        let difference = self - other;
        2 * difference * ratio * ratio * ratio -
            3 * difference * ratio * ratio +
            self
    }

    fn interpolate_exponential(&self, ratio: f64, exponent: f64, other: &f32) -> Self::Output {
        let a = -1 + self;
        (ratio.pow(exponent) * ((other - a) as f64).ln()).exp() + a
    }
}

impl InterpolatablePrimitive<f64> for f64 {
    type Output = f64;

    fn interpolate_nearest(&self, ratio: f64, other: &f64) -> Self::Output {
        if ratio < 0.5 {
            *self
        } else {
            *other
        }
    }

    fn interpolate_linear(&self, ratio: f64, other: &f64) -> Self::Output {
        (other - self) * ratio + self
    }

    fn interpolate_cubic(&self, ratio: f64, other: &f64) -> Self::Output {
        let difference = self - other;
        2 * difference * ratio * ratio * ratio -
            3 * difference * ratio * ratio +
            self
    }

    fn interpolate_exponential(&self, ratio: f64, exponent: f64, other: &f64) -> Self::Output {
        let a = -1 + self;
        (ratio.pow(exponent) * ((other - a) as f64).ln()).exp() + a
    }
}

impl InterpolatablePrimitive<d128> for d128 {
    type Output = d128;

    fn interpolate_nearest(&self, ratio: f64, other: &d128) -> Self::Output {
        if ratio < 0.5 {
            *self
        } else {
            *other
        }
    }

    fn interpolate_linear(&self, ratio: f64, other: &d128) -> Self::Output {
        (other - self) * ratio + self
    }

    fn interpolate_cubic(&self, ratio: f64, other: &d128) -> Self::Output {
        let difference = self - other;
        2 * difference * ratio * ratio * ratio -
            3 * difference * ratio * ratio +
            self
    }

    fn interpolate_exponential(&self, ratio: f64, exponent: f64, other: &d128) -> Self::Output {
        let a = -1 + self;
        (ratio.pow(exponent) * ((other - a) as f64).ln()).exp() + a
    }
}

impl Interpolatable<i8> for i8 {
    type Output = i8;

    fn interpolate(&self, interpolator: &Interpolator, other: &i8) -> Self::Output {
        interpolator.interpolate_primitive(self, other)
    }
}

impl Interpolatable<i16> for i16 {
    type Output = i16;

    fn interpolate(&self, interpolator: &Interpolator, other: &i16) -> Self::Output {
        interpolator.interpolate_primitive(self, other)
    }
}

impl Interpolatable<i32> for i32 {
    type Output = i32;

    fn interpolate(&self, interpolator: &Interpolator, other: &i32) -> Self::Output {
        interpolator.interpolate_primitive(self, other)
    }
}

impl Interpolatable<i64> for i64 {
    type Output = i64;

    fn interpolate(&self, interpolator: &Interpolator, other: &i64) -> Self::Output {
        interpolator.interpolate_primitive(self, other)
    }
}

impl Interpolatable<i128> for i128 {
    type Output = i128;

    fn interpolate(&self, interpolator: &Interpolator, other: &i128) -> Self::Output {
        interpolator.interpolate_primitive(self, other)
    }
}

impl Interpolatable<u8> for u8 {
    type Output = u8;

    fn interpolate(&self, interpolator: &Interpolator, other: &u8) -> Self::Output {
        interpolator.interpolate_primitive(self, other)
    }
}

impl Interpolatable<u16> for u16 {
    type Output = u16;

    fn interpolate(&self, interpolator: &Interpolator, other: &u16) -> Self::Output {
        interpolator.interpolate_primitive(self, other)
    }
}

impl Interpolatable<u32> for u32 {
    type Output = u32;

    fn interpolate(&self, interpolator: &Interpolator, other: &u32) -> Self::Output {
        interpolator.interpolate_primitive(self, other)
    }
}

impl Interpolatable<u64> for u64 {
    type Output = u64;

    fn interpolate(&self, interpolator: &Interpolator, other: &u64) -> Self::Output {
        interpolator.interpolate_primitive(self, other)
    }
}

impl Interpolatable<u128> for u128 {
    type Output = u128;

    fn interpolate(&self, interpolator: &Interpolator, other: &u128) -> Self::Output {
        interpolator.interpolate_primitive(self, other)
    }
}

impl Interpolatable<f32> for f32 {
    type Output = f32;

    fn interpolate(&self, interpolator: &Interpolator, other: &f32) -> Self::Output {
        interpolator.interpolate_primitive(self, other)
    }
}

impl Interpolatable<f64> for f64 {
    type Output = f64;

    fn interpolate(&self, interpolator: &Interpolator, other: &f64) -> Self::Output {
        interpolator.interpolate_primitive(self, other)
    }
}

impl Interpolatable<d128> for d128 {
    type Output = d128;

    fn interpolate(&self, interpolator: &Interpolator, other: &d128) -> Self::Output {
        interpolator.interpolate_primitive(self, other)
    }
}

pub struct InterpolatableLocation<R, T: Interpolatable<R>> {
    pub(crate) value: T,
    pub(crate) location: f64,
}

impl<R, T: Interpolatable<R>> Interpolatable<InterpolatableLocation<R, T>>
for InterpolatableLocation<R, T> {
    type Output = T::Output;

    fn interpolate(&self, interpolator: &Interpolator, other: &InterpolatableLocation<R, T>) -> Self::Output {
        let range = other.location - self.location;
        let relative_location = interpolator.ratio - self.location;

        let mut interpolator = Interpolator::new(interpolator.interpolation);

        if range == 0f64 {
            interpolator.ratio = 0.5f64;
            interpolator.interpolate(&self.value, &other.value)
        } else {
            let scaled_ratio = relative_location / range;
            interpolator.ratio = scaled_ratio;
            interpolator.interpolate(&self.value, &other.value)
        }
    }
}