use super::{Norm, Tau, Z, ZTau};
use num_rational::Rational64;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Default, Ord, PartialOrd)]
pub struct R(pub Rational64);
impl Add<Self> for R {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}
impl Sub<Self> for R {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}
impl Mul<Self> for R {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}
impl Div<Self> for R {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0)
    }
}
impl Neg for R {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl From<&Z> for R {
    fn from(v: &Z) -> Self {
        R(Rational64::new(v.0, 1))
    }
}

impl From<&(Z, Z)> for R {
    fn from(v: &(Z, Z)) -> Self {
        R(Rational64::new(v.0.0, v.1.0))
    }
}

// Rational ring in terms of characteristic polynomial of K-233 curve, R[\tau] = R[X] / \tau^2 - \mu * \tau + 2
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RTau {
    pub a0: R,
    pub a1: R,
}

impl RTau {
    pub fn new(a0: R, a1: R) -> Self {
        Self { a0, a1 }
    }
}

// Norm of R[\tau]
impl Norm<R> for RTau {
    fn norm(&self) -> R {
        let (a0_square, a0_mul_a1, a1_square) =
            (self.a0 * self.a0, self.a0 * self.a1, self.a1 * self.a1);
        a0_square + a0_mul_a1 * R::from(&Self::MU) + R::from(&Self::CHAR) * a1_square
    }
}

impl From<&ZTau> for RTau {
    fn from(zt: &ZTau) -> Self {
        RTau::new(R::from(&zt.a0), R::from(&zt.a1))
    }
}

impl Tau for RTau {
    fn one() -> Self {
        Self::from(&ZTau::one())
    }
    fn zero() -> Self {
        Self::from(&ZTau::zero())
    }
    fn automorphism(&self) -> Self {
        unimplemented!()
    }
    fn isomorphism(&self, _: Z) -> Z {
        unimplemented!()
    }
    fn conjugate(&self) -> Self {
        unimplemented!()
    }
}
/////////////////////////////////////////////////////////////////////////////////////////////////////
