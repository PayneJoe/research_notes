/// Scalar multiplication of K-233 group element with Koblitz tau expansion tricks on scalar field
///
use num_rational::Rational64;
use std::ops::{Add, Div, Mul, Neg, Sub};

// pub type Z = i64;
// pub type R = Rational64;
#[derive(Clone, Copy, Debug, Eq, PartialEq, Default, Ord, PartialOrd)]
pub struct Z(i64);
#[derive(Clone, Copy, Debug, Eq, PartialEq, Default, Ord, PartialOrd)]
pub struct R(Rational64);

impl Add<Self> for Z {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}
impl Sub<Self> for Z {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}
impl Mul<Self> for Z {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}
impl Div<Self> for Z {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0)
    }
}
impl Neg for Z {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}
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

// Round off Algorithm specially for Koblitz tau
impl From<&R> for Z {
    fn from(v: &R) -> Self {
        let half = Rational64::new(1, 2);
        if v.0.numer().signum() * v.0.denom().signum() > 0 {
            Z(((v.0 - half).ceil()).to_integer())
        } else {
            Z((v.0 + half).floor().to_integer())
        }
    }
}

// N(x) = \prod_i x * x_i, where x_i are all the conjugates of x
// For example, if x is a complex number, then there are only two conjugative numbers
pub trait Norm<T> {
    fn norm(&self) -> T;
}

// Characteristic polynomial of Frobenius Endomorphism for binary curve K-233: \tau^2 - \mu * \tau + 2
// \mu = (-1)^{1 - a_2}, \mu = -1 when a_2 = 0 in K-233 curve
pub trait Tau {
    const MU: Z = Z(-1);
    const CHAR: Z = Z(2);
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

impl Tau for RTau {}

// Integer ring in terms of characteristic polynomial of K-233 curve, Z[\tau] = Z[X] / \tau^2 - \mu * \tau + 2
#[derive(Clone, Copy, Eq, PartialEq, Debug, Default)]
pub struct ZTau {
    pub a0: Z,
    pub a1: Z,
}

// Norm of Z[\tau]
impl Norm<Z> for ZTau {
    fn norm(&self) -> Z {
        let x0 = self.a0 * self.a0 + Self::CHAR * self.a1 * self.a1;
        let x1 = Self::MU * self.a0 * self.a1;
        x0 + x1
    }
}

impl From<&RTau> for ZTau {
    // refer to "handbook of elliptic and hyperelliptic curve cryptography", Algorithm 15.9
    fn from(rt: &RTau) -> Self {
        let (f0, f1) = (Z::from(&rt.a0), Z::from(&rt.a1));
        let (eta0, eta1) = (rt.a0 - R::from(&f0), rt.a1 - R::from(&f1));
        let (mut h0, mut h1) = (Z::default(), Z::default());
        let (one, two, three, four) = (
            R::from(&Z(1)),
            R::from(&Z(2)),
            R::from(&Z(3)),
            R::from(&Z(4)),
        );
        let r_mu = R::from(&Self::MU);
        let eta = two * eta0 + eta1 * r_mu;
        if eta >= one {
            if eta0 - three * r_mu * eta1 < -one {
                h1 = Self::MU;
            } else {
                h0 = Z(1);
            }
        } else {
            if eta0 + four * r_mu * eta1 >= two {
                h1 = Self::MU;
            }
        }
        if eta < -one {
            if eta0 - three * r_mu * eta1 >= one {
                h1 = -Self::MU;
            } else {
                h0 = -Z(1);
            }
        } else {
            if eta0 + four * r_mu * eta1 < -two {
                h1 = -Self::MU;
            }
        }
        let (a0, a1) = (f0 + h0, f1 + h1);
        ZTau { a0, a1 }
    }
}

impl Tau for ZTau {}
