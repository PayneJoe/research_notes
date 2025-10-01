use core::ops::{Add, Div, Mul, Sub};

use num_rational::Rational64;

use crate::rational_quadratic::{RationalQuadraticField, Round};

type BaseField = i64;
pub const MU: BaseField = -1;

#[derive(Clone, Copy)]
pub struct IntegerQuadraticField {
    pub a0: BaseField,
    pub a1: BaseField,
}

impl IntegerQuadraticField {
    pub fn norm(self) -> BaseField {
        let a0 = self.a0 * self.a0 + 2 * self.a1 * self.a1;
        let a1 = MU * self.a0 * self.a1;
        a0 + a1
    }

    pub fn conjugate(self) -> Self {
        Self {
            a0: self.a0 + MU * self.a1,
            a1: -self.a1,
        }
    }

    pub fn mul_conj(self, other: Self) -> Self {
        let a0 = self.a0 * other.a0 + 2 * self.a1 * other.a1 + MU * self.a0 * other.a1;
        let a1 = self.a1 * other.a0 - self.a0 * other.a1;
        Self { a0, a1 }
    }
}

impl Div for IntegerQuadraticField {
    type Output = (Self, Self);

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
        let a0 = self.a0 * other.a0 - 2 * self.a1 * other.a1;
        let a1 = self.a0 * other.a1 + self.a1 * other.a0 + MU * self.a1 * other.a1;
        Self { a0, a1 }
    }
}
