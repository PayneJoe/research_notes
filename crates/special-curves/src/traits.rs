use core::ops::{Add, Div, Mul, Sub};
use num_rational::Rational64;

/// Trait for rounding off a rational number to the nearest integer
pub trait Round {
    type Output;
    fn round_off(&self) -> Self::Output;
}

pub trait Norm {
    fn norm(&self) -> i64;
}

impl Round for Rational64 {
    type Output = i64;
    fn round_off(&self) -> Self::Output {
        let half = Rational64::new(1, 2);
        if self.numer().signum() * self.denom().signum() > 0 {
            ((self.clone() - half).ceil()).to_integer()
        } else {
            (self.clone() + half).floor().to_integer()
        }
    }
}

pub trait QuadraticBasicArithmetics:
    Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Div<Output = Self> + Sized
{
}

pub trait IntegerQuadraticTraits: QuadraticBasicArithmetics + Norm {}

pub trait IntegerQuadraticConjugateTraits: QuadraticBasicArithmetics {}
