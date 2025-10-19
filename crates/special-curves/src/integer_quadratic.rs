use crate::Modulos;
use crate::rational_quadratic::RationalQuadraticField;
use crate::{AsRational, Norm, Round};
use core::ops::{Add, Div, Mul, Sub};
use num_rational::Rational64;

pub type IntegerBaseField = i64;

impl Modulos for IntegerBaseField {
    fn modulos(&self, modulus: Self) -> Self {
        let mut r = *self % modulus;
        if r < 0 {
            r = r + modulus;
        }
        r
    }

    // naive implementation of computing x^-1 mod 2^k, where x must be odd
    fn inv_mod_pow_k(&self, k: usize) -> Self {
        let modulus = 1 << k;
        let r = self.modulos(modulus);
        assert!(r % 2 == 1, "only odd integers have inverses mod 2^k");
        let mut s = 1;
        let mut result = 0;
        while s < modulus {
            if (s * modulus + 1) % self == 0 {
                result = ((s * modulus + 1) / self).modulos(modulus);
            }
            s += 1;
        }
        result
    }
}

/// Curve function: y^2 + xy = x^3 + a_2 * x^2 + 1, with a_2 = 0, \mu = (-1)^{1 - a_2}
/// 1) \mu = -1, when a_2 = 0
/// 2) \mu = 1, when a_2 = 1
///
/// Characteristic polynomial of the Frobenius Endomorphism: T^2 - \mu * \tau + 2 \equiv 0
/// 1) \tau + \bar{\tau} = \mu
/// 2) \tau * \bar{\tau} = bias = 2
// pub const MU: IntegerBaseField = -1;
pub const MU: IntegerBaseField = 1;
pub const BIAS: IntegerBaseField = 2;

impl AsRational for IntegerBaseField {
    type Output = Rational64;
    fn as_rational(&self) -> Self::Output {
        Rational64::new(*self, 1)
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug, Default)]
pub struct IntegerQuadraticField {
    pub a0: IntegerBaseField,
    pub a1: IntegerBaseField,
}

impl AsRational for IntegerQuadraticField {
    type Output = RationalQuadraticField;

    fn as_rational(&self) -> Self::Output {
        RationalQuadraticField::new(self.a0.as_rational(), self.a1.as_rational())
    }
}

impl Norm for IntegerQuadraticField {
    type Output = IntegerBaseField;

    fn norm(&self) -> Self::Output {
        let x0 = self.a0 * self.a0 + BIAS * self.a1 * self.a1;
        let x1 = MU * self.a0 * self.a1;
        x0 + x1
    }
}

impl IntegerQuadraticField {
    pub fn new(a0: IntegerBaseField, a1: IntegerBaseField) -> Self {
        Self { a0, a1 }
    }

    pub fn one() -> Self {
        Self::new(1, 0)
    }

    pub fn zero() -> Self {
        Self::new(0, 0)
    }

    pub fn conjugate(&self) -> Self {
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

    pub fn add_conj(self, other: Self) -> Self {
        Self {
            a0: self.a0 + other.a0 + MU * other.a1,
            a1: self.a1 - other.a1,
        }
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
