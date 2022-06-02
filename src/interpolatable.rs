use std::marker::PhantomData;
use std::str::FromStr;

use decimal::d128;

#[derive(Copy, Clone)]
pub enum Interpolation {
    LINEAR,
    CUBIC,
    NEAREST,
    EASING(f64),
}

impl Interpolation {
    pub fn interpolator(self) -> Interpolator {
        Interpolator::new(self)
    }

    pub fn interpolate<R, T: Interpolatable<R>>(self, first: &T, ratio: f64, second: &R) -> T::Output {
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
            Interpolation::EASING(n) => {
                this.interpolate_ease(self.ratio, n, other)
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
    fn interpolate_ease(&self, ratio: f64, bias: f64, other: &R) -> Self::Output;
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
        (((other - self) as f64) * ratio) as i8 + self
    }

    fn interpolate_cubic(&self, ratio: f64, other: &i8) -> Self::Output {
        let difference = (self - other) as f64;
        (2f64 * difference * ratio * ratio * ratio -
            3f64 * difference * ratio * ratio) as i8 +
            self
    }

    fn interpolate_ease(&self, ratio: f64, bias: f64, other: &i8) -> Self::Output {
        let k = bias.exp() - 1f64;
        (((other - self) as f64 * (k * ratio + ratio)) / (k * ratio + 1f64)) as i8 + self
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
        (((other - self) as f64) * ratio) as i16 + self
    }

    fn interpolate_cubic(&self, ratio: f64, other: &i16) -> Self::Output {
        let difference = (self - other) as f64;
        (2f64 * difference * ratio * ratio * ratio -
            3f64 * difference * ratio * ratio) as i16 +
            self
    }

    fn interpolate_ease(&self, ratio: f64, bias: f64, other: &i16) -> Self::Output {
        let k = bias.exp() - 1f64;
        (((other - self) as f64 * (k * ratio + ratio)) / (k * ratio + 1f64)) as i16 + self
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
        (((other - self) as f64) * ratio) as i32 + self
    }

    fn interpolate_cubic(&self, ratio: f64, other: &i32) -> Self::Output {
        let difference = (self - other) as f64;
        (2f64 * difference * ratio * ratio * ratio -
            3f64 * difference * ratio * ratio) as i32 +
            self
    }

    fn interpolate_ease(&self, ratio: f64, bias: f64, other: &i32) -> Self::Output {
        let k = bias.exp() - 1f64;
        (((other - self) as f64 * (k * ratio + ratio)) / (k * ratio + 1f64)) as i32 + self
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
        (((other - self) as f64) * ratio) as i64 + self
    }

    fn interpolate_cubic(&self, ratio: f64, other: &i64) -> Self::Output {
        let difference = (self - other) as f64;
        (2f64 * difference * ratio * ratio * ratio -
            3f64 * difference * ratio * ratio) as i64 +
            self
    }

    fn interpolate_ease(&self, ratio: f64, bias: f64, other: &i64) -> Self::Output {
        let k = bias.exp() - 1f64;
        (((other - self) as f64 * (k * ratio + ratio)) / (k * ratio + 1f64)) as i64 + self
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
        (((other - self) as f64) * ratio) as i128 + self
    }

    fn interpolate_cubic(&self, ratio: f64, other: &i128) -> Self::Output {
        let difference = (self - other) as f64;
        (2f64 * difference * ratio * ratio * ratio -
            3f64 * difference * ratio * ratio) as i128 +
            self
    }

    fn interpolate_ease(&self, ratio: f64, bias: f64, other: &i128) -> Self::Output {
        let k = bias.exp() - 1f64;
        (((other - self) as f64 * (k * ratio + ratio)) / (k * ratio + 1f64)) as i128 + self
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
        (((other - self) as f64) * ratio) as u8 + self
    }

    fn interpolate_cubic(&self, ratio: f64, other: &u8) -> Self::Output {
        let difference = (self - other) as f64;
        (2f64 * difference * ratio * ratio * ratio -
            3f64 * difference * ratio * ratio) as u8 +
            self
    }

    fn interpolate_ease(&self, ratio: f64, bias: f64, other: &u8) -> Self::Output {
        let k = bias.exp() - 1f64;
        (((other - self) as f64 * (k * ratio + ratio)) / (k * ratio + 1f64)) as u8 + self
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
        (((other - self) as f64) * ratio) as u16 + self
    }

    fn interpolate_cubic(&self, ratio: f64, other: &u16) -> Self::Output {
        let difference = (self - other) as f64;
        (2f64 * difference * ratio * ratio * ratio -
            3f64 * difference * ratio * ratio) as u16 +
            self
    }

    fn interpolate_ease(&self, ratio: f64, bias: f64, other: &u16) -> Self::Output {
        let k = bias.exp() - 1f64;
        (((other - self) as f64 * (k * ratio + ratio)) / (k * ratio + 1f64)) as u16 + self
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
        (((other - self) as f64) * ratio) as u32 + self
    }

    fn interpolate_cubic(&self, ratio: f64, other: &u32) -> Self::Output {
        let difference = (self - other) as f64;
        (2f64 * difference * ratio * ratio * ratio -
            3f64 * difference * ratio * ratio) as u32 +
            self
    }

    fn interpolate_ease(&self, ratio: f64, bias: f64, other: &u32) -> Self::Output {
        let k = bias.exp() - 1f64;
        (((other - self) as f64 * (k * ratio + ratio)) / (k * ratio + 1f64)) as u32 + self
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
        (((other - self) as f64) * ratio) as u64 + self
    }

    fn interpolate_cubic(&self, ratio: f64, other: &u64) -> Self::Output {
        let difference = (self - other) as f64;
        (2f64 * difference * ratio * ratio * ratio -
            3f64 * difference * ratio * ratio) as u64 +
            self
    }

    fn interpolate_ease(&self, ratio: f64, bias: f64, other: &u64) -> Self::Output {
        let k = bias.exp() - 1f64;
        (((other - self) as f64 * (k * ratio + ratio)) / (k * ratio + 1f64)) as u64 + self
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
        (((other - self) as f64) * ratio) as u128 + self
    }

    fn interpolate_cubic(&self, ratio: f64, other: &u128) -> Self::Output {
        let difference = (self - other) as f64;
        (2f64 * difference * ratio * ratio * ratio -
            3f64 * difference * ratio * ratio) as u128 +
            self
    }

    fn interpolate_ease(&self, ratio: f64, bias: f64, other: &u128) -> Self::Output {
        let k = bias.exp() - 1f64;
        (((other - self) as f64 * (k * ratio + ratio)) / (k * ratio + 1f64)) as u128 + self
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
        (((other - self) as f64) * ratio) as f32 + self
    }

    fn interpolate_cubic(&self, ratio: f64, other: &f32) -> Self::Output {
        let difference = (self - other) as f64;
        (2f64 * difference * ratio * ratio * ratio -
            3f64 * difference * ratio * ratio) as f32 +
            self
    }

    fn interpolate_ease(&self, ratio: f64, bias: f64, other: &f32) -> Self::Output {
        let k = bias.exp() - 1f64;
        (((other - self) as f64 * (k * ratio + ratio)) / (k * ratio + 1f64)) as f32 + self
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
        (((other - self) as f64) * ratio) as f64 + self
    }

    fn interpolate_cubic(&self, ratio: f64, other: &f64) -> Self::Output {
        let difference = (self - other) as f64;
        (2f64 * difference * ratio * ratio * ratio -
            3f64 * difference * ratio * ratio) as f64 +
            self
    }

    fn interpolate_ease(&self, ratio: f64, bias: f64, other: &f64) -> Self::Output {
        let k = bias.exp() - 1f64;
        ((other - self) * (k * ratio + ratio)) / (k * ratio + 1f64) + self
        // let k = bias.exp();
        // ((other - self) * ratio * factor) / (ratio * k - ratio + 1f64) + self
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
        ((other - self) * d128::from_str(&*ratio.to_string()).unwrap()) + self
    }

    fn interpolate_cubic(&self, ratio: f64, other: &d128) -> Self::Output {
        let ratio = d128::from_str(ratio.to_string().as_str()).unwrap();
        let difference = self - other;
        d128!(2) * difference * ratio * ratio * ratio -
            d128!(3) * difference * ratio * ratio +
            self
    }

    fn interpolate_ease(&self, ratio: f64, bias: f64, other: &d128) -> Self::Output {
        let k = d128::from_str(&*(bias.exp() - 1f64).to_string()).unwrap();
        let ratio = d128::from_str(&*ratio.to_string()).unwrap();
        ((other - self) * (k * ratio + ratio)) / (k * ratio + d128!(1)) + self
    }
}

impl InterpolatablePrimitive<bool> for bool {
    type Output = bool;

    fn interpolate_nearest(&self, ratio: f64, other: &bool) -> Self::Output {
        if ratio < 0.5 {
            *self
        } else {
            *other
        }
    }

    fn interpolate_linear(&self, ratio: f64, other: &bool) -> Self::Output {
        self.interpolate_nearest(ratio, other)
    }

    fn interpolate_cubic(&self, ratio: f64, other: &bool) -> Self::Output {
        self.interpolate_nearest(ratio, other)
    }

    fn interpolate_ease(&self, ratio: f64, bias: f64, other: &bool) -> Self::Output {
        let k = bias.exp() - 1f64;
        self.interpolate_nearest((k * ratio + ratio) / (k * ratio + 1f64), other)
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

impl Interpolatable<bool> for bool {
    type Output = bool;

    fn interpolate(&self, interpolator: &Interpolator, other: &bool) -> Self::Output {
        interpolator.interpolate_primitive(self, other)
    }
}

pub struct InterpolatableLocation<'a, R, T: Interpolatable<R>> {
    pub(crate) value: &'a T,
    pub(crate) location: f64,
    type_r: PhantomData<R>,
}

impl<R, T: Interpolatable<R>> InterpolatableLocation<'_, R, T> {
    pub fn new(value: &T, location: f64) -> InterpolatableLocation<R, T> {
        InterpolatableLocation {
            value,
            location,
            type_r: PhantomData,
        }
    }
}

impl<C, R: Interpolatable<C>, T: Interpolatable<R>> Interpolatable<InterpolatableLocation<'_, C, R>>
for InterpolatableLocation<'_, R, T> {
    type Output = T::Output;

    fn interpolate(&self, interpolator: &Interpolator, other: &InterpolatableLocation<C, R>) -> Self::Output {
        let range = other.location - self.location;
        let relative_location = interpolator.ratio - self.location;

        let mut interpolator = Interpolator::new(interpolator.interpolation);

        if range == 0f64 {
            interpolator.ratio = 0.5f64;
            interpolator.interpolate(self.value, other.value)
        } else {
            let scaled_ratio = relative_location / range;
            interpolator.ratio = scaled_ratio;
            interpolator.interpolate(self.value, other.value)
        }
    }
}