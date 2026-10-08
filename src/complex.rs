use std::fmt;
use std::ops::{Add, Sub, Mul, Div, Neg};

use crate::scalar::Field;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Complex {
    pub re: f32,
    pub im: f32,
}

impl Complex {
    pub fn new(re: f32, im: f32) -> Self {
        Self { re, im }
    }
}

impl From<f32> for Complex {
    fn from(value: f32) -> Self {
        Complex {
            re: value,
            im: 0.0,
        }
    }
}

impl Add for Complex {
    type Output = Complex;

    fn add(self, other: Complex) -> Complex {
        Complex {
            re: self.re + other.re,
            im: self.im + other.im,
        }
    }
}

impl Sub for Complex {
    type Output = Complex;

    fn sub(self, other: Complex) -> Complex {
        Complex {
            re: self.re - other.re,
            im: self.im - other.im,
        }
    }
}

impl Mul for Complex {
    type Output = Complex;

    fn mul(self, other: Complex) -> Complex {
        Complex {
            re: self.re * other.re - self.im * other.im,
            im: self.re * other.im + self.im * other.re,
        }
    }
}

impl Div for Complex {
    type Output = Complex;

    fn div(self, other: Complex) -> Complex {
        let denom = other.re * other.re + other.im * other.im;
        if denom == 0.0 {
            panic!("Division by zero in complex division");
        }
        Complex {
            re: (self.re * other.re + self.im * other.im) / denom,
            im: (self.im * other.re - self.re * other.im) / denom,
        }
    }
}

impl Mul<f32> for Complex {
    type Output = Complex;

    fn mul(self, scalar: f32) -> Complex {
        Complex {
            re: self.re * scalar,
            im: self.im * scalar,
        }
    }
}

impl Div<f32> for Complex {
    type Output = Complex;

    fn div(self, scalar: f32) -> Complex {
        if scalar == 0.0 {
            panic!("Division by zero in complex-scalar division");
        }
        Complex {
            re: self.re / scalar,
            im: self.im / scalar,
        }
    }
}

impl Neg for Complex {
    type Output = Complex;

    fn neg(self) -> Complex {
        Complex {
            re: -self.re,
            im: -self.im,
        }
    }
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.im >= 0.0 {
            write!(f, "{} + {}i", self.re, self.im)
        } else {
            write!(f, "{} - {}i", self.re, -self.im)
        }
    }
}

impl Field for Complex {
    fn zero() -> Self {
        Complex { re: 0.0, im: 0.0 }
    }

    fn one() -> Self {
        Complex { re: 1.0, im: 0.0 }
    }

    fn conj(&self) -> Self {
        Complex { re: self.re, im: -self.im }
    }

    fn abs_square(&self) -> f32 {
        self.re * self.re + self.im * self.im
    }

    fn is_zero(&self) -> bool {
        const EPS: f32 = 1e-6;
        self.re.abs() < EPS && self.im.abs() < EPS
    }

    fn angle_numerator(&self) -> f32 {
        self.magnitude()
    }
}