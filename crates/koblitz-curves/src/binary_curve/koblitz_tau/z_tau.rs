use super::{LucasSequence, Norm, R, RTau, Tau};
use num_rational::Rational64;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Default, Ord, PartialOrd)]
pub struct Z(pub i64);

impl From<Z> for usize {
    fn from(v: Z) -> Self {
        v.0.abs() as usize
    }
}

impl Z {
    fn signum(&self) -> Self {
        Z(self.0.signum())
    }

    fn abs(&self) -> Self {
        Z(self.0.abs())
    }

    // abs(x) = +1 or -1
    fn is_odd(&self) -> bool {
        (self.0 % 2).abs() == 1
    }

    // reduce to a postive integer with a modulus
    pub fn reduce(&self, modulus: Self) -> Self {
        let mut r = self.0 % modulus.0;
        if r < 0 {
            r = r + modulus.0;
        }
        Self(r)
    }
    // !!! Naitve implementation of inverse mod a modulus
    pub fn inv_mod(&self, modulus: Self) -> Self {
        let r = self.reduce(modulus);
        assert!(r.is_odd(), "only odd integers have inverses mod 2^k");
        let mut s = Z(1);
        let mut result = Z(0);
        while s < modulus {
            let v = s * modulus + Z(1);
            if v.reduce(r) == Z(0) {
                result = (v / r).reduce(modulus);
            }
            s = s + Z(1);
        }
        result
    }
}

impl From<&R> for Z {
    // Tau-friendly round off algorithm
    fn from(v: &R) -> Self {
        let half = Rational64::new(1, 2);
        if v.0.numer().signum() * v.0.denom().signum() > 0 {
            Z(((v.0 - half).ceil()).to_integer())
        } else {
            Z((v.0 + half).floor().to_integer())
        }
    }
}

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

///////////////////////////////////////////////////////////////////////////////////////////////////// Integer tau expansion
// Integer ring in terms of characteristic polynomial of K-233 curve, Z[\tau] = Z / \tau^2 - \mu * \tau + 2
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct ZTau {
    pub a0: Z,
    pub a1: Z,
}

impl Default for ZTau {
    fn default() -> Self {
        Self::new(Z(0), Z(1))
    }
}

impl ZTau {
    pub fn new(a0: Z, a1: Z) -> Self {
        Self { a0, a1 }
    }

    pub fn is_zero(&self) -> bool {
        *self == Self::zero()
    }

    // reduce with a modulus, usually we use \tau^w
    pub fn reduce(&self, modulus: Self) -> Self {
        (*self / modulus).1
    }

    // convert Z[\tau] to tauNAF expansion
    pub fn tauNAF(&self) -> Vec<Z> {
        let mut result = vec![];
        let (mut n0, mut n1) = (self.a0, self.a1);
        while n0.abs() + n1.abs() != Z(0) {
            let ri = if n0.is_odd() {
                // ensure r_i = +1 or -1, since (n0 - 2 * n1) % 4 = 1 or 3
                let residual = Z(2) - (n0 - Z(2) * n1).reduce(Z(4));
                n0 = n0 - residual;
                residual
            } else {
                Z(0)
            };
            result.push(ri);
            (n0, n1) = (n1 + Self::MU * n0 / Z(2), -n0 / Z(2));
        }
        result
    }
    // convert Z[\tau] to tauNAF_w expansion
    pub fn tauNAFw(&self, w: usize) -> Vec<Z> {
        let h_w = Self::h_w(w);
        let (u_mod_tau_w, alpha_u) = Self::precomputed_table(w);
        let mut result = vec![];
        // let (mut n0, mut n1) = (self.a0, self.a1);
        let mut t = self.clone();
        while t.is_zero() == false {
            let ri = if t.a0.is_odd() {
                let u = t.isomorphism(h_w);
                let residual = u_mod_tau_w[usize::from(u)];
                t = if u > Z(0) { t - residual } else { t + residual };
                alpha_u[usize::from(u)].clone()
            } else {
                [Z(0)].to_vec()
            };
            result.extend(ri.into_iter());
            // now t is a even, then right shift w times, i.e. t = t / \tau^w
            t = Self::new(t.a1 + Self::MU * t.a0 / Z(2), -t.a0 / Z(2));
        }
        result
    }
}

// Norm of Z[\tau]
impl Norm<Z> for ZTau {
    fn norm(&self) -> Z {
        let x0 = self.a0 * self.a0 + Self::CHAR * self.a1 * self.a1;
        let x1 = Self::MU * self.a0 * self.a1;
        x0 + x1
    }
}

// refer to "handbook of elliptic and hyperelliptic curve cryptography", Algorithm 15.9
// find a optimal ZTau which is the closest in terms of lattice distance to RTau
impl From<&RTau> for ZTau {
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

impl From<&LucasSequence> for ZTau {
    fn from(s: &LucasSequence) -> Self {
        Self::new(-Self::CHAR * s.u0, s.u1)
    }
}

impl Tau for ZTau {
    fn one() -> Self {
        Self { a0: Z(1), a1: Z(0) }
    }
    fn zero() -> Self {
        Self { a0: Z(0), a1: Z(0) }
    }
    // Z[\bar{\tau}] -> Z[\tau]
    fn automorphism(&self) -> Self {
        Self {
            a0: self.a0 + Self::MU * self.a1,
            a1: -self.a1,
        }
    }
    // Z[\tau] -> Z[2^w]
    fn isomorphism(&self, h_w: Z) -> Z {
        self.a0 + self.a1 * h_w
    }
}

impl Add<Self> for ZTau {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            a0: self.a0 + rhs.a0,
            a1: self.a1 + rhs.a1,
        }
    }
}

impl Sub<Self> for ZTau {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            a0: self.a0 - rhs.a0,
            a1: self.a1 - rhs.a1,
        }
    }
}

// (a + b * \tau) * (c + d * \tau)
impl Mul<Self> for ZTau {
    type Output = Self;
    fn mul(self, rht: Self) -> Self::Output {
        let a0 = self.a0 * rht.a0 - Self::CHAR * self.a1 * rht.a1;
        let a1 = self.a0 * rht.a1 + self.a1 * rht.a0 + Self::MU * self.a1 * rht.a1;
        Self { a0, a1 }
    }
}

// (a + b * \tau) / N = a / N + b / N * \tau
impl Div<Z> for ZTau {
    type Output = RTau;
    fn div(self, rht: Z) -> Self::Output {
        RTau::new(R::from(&(self.a0, rht)), R::from(&(self.a1, rht)))
    }
}

// refer to "Handbook of Elliptic and Hyperelliptic Curve Cryptography", Algorithm 15.11
// (a + b * \tau) / (c + d * \tau)
// = (a + b * \tau) * (c + d * \bar{tau}) / N(c + d * \tau)
impl Div<Self> for ZTau {
    type Output = (Self, Self);
    fn div(self, rht: Self) -> Self::Output {
        let N = rht.norm();
        let g = self * rht.conjugate();
        let g_div_N = g / N;
        let k = ZTau::from(&g_div_N);
        let ro = self - k * rht;
        (k, ro)
    }
}
//////////////////////////////////////////////////////////////////////////////////////////////////////////

#[allow(deprecated)]
#[cfg(test)]
mod tests {
    use super::*;
    use rand::{self, Rng};

    #[test]
    fn test_r_to_z() {
        let r1 = R(Rational64::new(1, 2));
        let r2 = R(Rational64::new(-1, 2));
        assert!(
            (Z::from(&r1) == Z(0)) && (Z::from(&r2) == Z(0)),
            "Test for round(1 / 2) == round(-1 / 2) == 0 failed!"
        );
    }

    #[test]
    fn test_rtau_to_ztau() {
        let u = RTau::new(R(Rational64::new(8, 5)), R(Rational64::new(12, 5)));
        let u_expected = ZTau::new(Z(1), Z(2));
        assert_eq!(ZTau::from(&u), u_expected, "Test for From(Rtau) failed!");
    }

    #[test]
    fn test_ztau_mul() {
        let (u, v, w_expected) = (
            ZTau::new(Z(1), Z(2)),
            ZTau::new(Z(2), Z(3)),
            ZTau::new(Z(-10), Z(1)),
        );
        let w = u * v;
        assert_eq!(w, w_expected, "Test for multiplication of ZTau failed!");
    }

    #[test]
    fn test_ztau_div() {
        let mut rng = rand::thread_rng();
        for _ in 0..1000 {
            let u = ZTau::new(
                Z(rng.gen_range(0..10000000i64)),
                Z(rng.gen_range(0..10000000i64)),
            );
            let v = ZTau::new(
                Z(rng.gen_range(0..10000000i64)),
                Z(rng.gen_range(0..10000000i64)),
            );
            let (_, ro) = u / v;
            assert!(
                Z(7) * ro.norm() < Z(4) * v.norm(),
                "Test for division of ZTau failed!"
            );
        }
    }

    #[test]
    fn test_tau_naf() {
        let u = ZTau::new(Z(409), Z(0));
        let w_expected = vec![
            Z(1),
            Z(0),
            Z(0),
            Z(-1),
            Z(0),
            Z(0),
            Z(1),
            Z(0),
            Z(-1),
            Z(0),
            Z(1),
            Z(0),
            Z(0),
            Z(0),
            Z(0),
            Z(1),
            Z(0),
            Z(0),
            Z(-1),
        ];
        let w = u.tauNAF();
        assert_eq!(w, w_expected, "Test for tauNAF failed!");
        let mut result = ZTau::zero();
        for w in (0..w_expected.len()).rev() {
            match w_expected[w] {
                Z(1) => result = result + ZTau::pow(w),
                Z(-1) => result = result - ZTau::pow(w),
                Z(0) => {}
                _ => unreachable!(),
            }
        }
        assert_eq!(result, u, "Reconfirmation of the tauNAF result failed!");
    }

    #[test]
    fn test_tau_pow() {
        let tau_powers = vec![
            ZTau::one(),
            ZTau::default(),
            ZTau::new(Z(-2), Z(-1)),
            ZTau::new(Z(2), Z(-1)),
            ZTau::new(Z(2), Z(3)),
        ];
        for w in 0..tau_powers.len() {
            assert_eq!(ZTau::pow(w), tau_powers[w]);
        }
    }

    #[test]
    fn test_tau_naf_w() {
        todo!()
    }
}
