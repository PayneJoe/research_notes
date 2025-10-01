use crate::rational_quadratic::RationalQuadraticField;
use crate::{AsRational, Norm, Round};
use core::ops::{Add, Div, Mul, Sub};
use num_rational::Rational64;

type BaseField = i64;
/// Curve function: y^2 + xy = x^3 + a_2 * x^2 + 1, with a_2 = 0, \mu = (-1)^{1 - a_2}
/// 1) \mu = -1, when a_2 = 0
/// 2) \mu = 1, when a_2 = 1
///
/// Characteristic polynomial of the Frobenius Endomorphism: T^2 - \mu * \tau + 2 \equiv 0
/// 1) \tau + \bar{\tau} = \mu
/// 2) \tau * \bar{\tau} = 2
pub const MU: BaseField = -1;
pub const BIAS: BaseField = 2;

impl AsRational for BaseField {
    type Output = Rational64;
    fn as_rational(&self) -> Self::Output {
        Rational64::new(*self, 1)
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct IntegerQuadraticField {
    pub a0: BaseField,
    pub a1: BaseField,
}

impl AsRational for IntegerQuadraticField {
    type Output = RationalQuadraticField;

    fn as_rational(&self) -> Self::Output {
        RationalQuadraticField::new(self.a0.as_rational(), self.a1.as_rational())
    }
}

impl Norm for IntegerQuadraticField {
    type Output = BaseField;

    fn norm(&self) -> Self::Output {
        let a0 = self.a0 * self.a0 + BIAS * self.a1 * self.a1;
        let a1 = MU * self.a0 * self.a1;
        a0 + a1
    }
}

impl IntegerQuadraticField {
    pub fn new(a0: BaseField, a1: BaseField) -> Self {
        Self { a0, a1 }
    }

    pub fn conjugate(self) -> Self {
        Self {
            a0: self.a0 + MU * self.a1,
            a1: -self.a1,
        }
    }

    pub fn mul_conj(self, other: Self) -> Self {
        let a0 = self.a0 * other.a0 + BIAS * self.a1 * other.a1 + MU * self.a0 * other.a1;
        let a1 = self.a1 * other.a0 - self.a0 * other.a1;
        Self { a0, a1 }
    }
}

impl Div for IntegerQuadraticField {
    type Output = (Self, Self);

    // refer to "Handbook of Elliptic and Hyperelliptic Curve Cryptography", Algorithm 15.11
    fn div(self, other: Self) -> Self::Output {
        let g = self * other.conjugate();
        let big_n = other.norm();
        let k =
            RationalQuadraticField::new(Rational64::new(g.a0, big_n), Rational64::new(g.a1, big_n))
                .round_off();
        let ro = self - k * other;
        (k, ro)
    }
}

impl Add for IntegerQuadraticField {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            a0: self.a0 + other.a0,
            a1: self.a1 + other.a1,
        }
    }
}

impl Sub for IntegerQuadraticField {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            a0: self.a0 - other.a0,
            a1: self.a1 - other.a1,
        }
    }
}

impl Mul for IntegerQuadraticField {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        let a0 = self.a0 * other.a0 - BIAS * self.a1 * other.a1;
        let a1 = self.a0 * other.a1 + self.a1 * other.a0 + MU * self.a1 * other.a1;
        Self { a0, a1 }
    }
}
