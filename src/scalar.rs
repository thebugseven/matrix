use::std::ops::{Add, Sub, Mul, Div};

pub trait Field:
    Clone
    + Default
    + PartialEq
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Mul<f32, Output = Self>
    + Div<f32, Output = Self>
{
    fn zero() -> Self;
    fn one() -> Self;

    fn conj(&self) -> Self;

    fn abs_square(&self) -> f32;

    fn is_zero(&self) -> bool;

    fn angle_numerator(&self) -> f32;

    fn magnitude(&self) -> f32 {
        self.abs_square().powf(0.5)
    }
}

impl Field for f32 {
    fn zero() -> Self {
        0.0
    }

    fn one() -> Self {
        1.0
    }

    fn conj(&self) -> Self {
        *self
    }

    fn abs_square(&self) -> f32 {
        *self * *self
    }

    fn is_zero(&self) -> bool {
        const EPS: f32 = 1e-6;

        *self > -EPS &&  *self < EPS
    }

    fn angle_numerator(&self) -> f32 {
        *self
    }
}