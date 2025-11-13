#![allow(unused)]

use crate::tau_expansion::integer_quadratic::{BIAS, IntegerBaseField, IntegerQuadraticField, MU};
use crate::tau_expansion::{Canonical, Modulos};
use core::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Copy, Clone, Debug)]
struct LucasSequence<const U0: IntegerBaseField = 0, const U1: IntegerBaseField = 1> {
    u0: IntegerBaseField,
    u1: IntegerBaseField,
}

impl<const U0: IntegerBaseField, const U1: IntegerBaseField> LucasSequence<U0, U1> {
    const fn new() -> Self {
        Self { u0: U0, u1: U1 }
    }

    // refer to "Handbook of Elliptic and Hyperelliptic Curve Cryptography", equation 15.4
    pub fn next(&self) -> Self {
        Self {
            u0: self.u1,
            u1: MU * self.u1 - BIAS * self.u0,
        }
    }

    pub fn as_tau_quadratic(&self) -> TauQuadratic {
        TauQuadratic::new(-BIAS * self.u0, self.u1)
    }
}

type TauLucasSequence = LucasSequence<0, 1>;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Tau<const W: usize>;

impl<const W: usize> Tau<W> {
    pub fn trace() -> IntegerBaseField {
        MU
    }

    pub fn determint() -> IntegerBaseField {
        BIAS
    }

    // \tau^d = U_d * \tau - U_{d - 1} * BIAS
    pub fn pow() -> TauQuadratic {
        if W == 0 {
            return TauQuadratic::one();
        }
        if W == 1 {
            return TauQuadratic::from_tau();
        }
        let mut result = TauLucasSequence::new();
        for _ in 1..W {
            result = result.next();
        }

        result.as_tau_quadratic()
    }

    // hw, a positive integer, is approximate of \tau mod 2^k
    // refer to "Handbook of Elliptic and Hyperelliptic Curve Cryptography" page 363
    pub fn hw() -> IntegerBaseField {
        let mut l = TauLucasSequence::new();
        for _ in 0..W - 1 {
            l = l.next();
        }
        let uk_inv = l.u1.inv_mod_pow_k(W);
        (2 * l.u0 * uk_inv).modulos(1 << W)
    }

    // refer to "Handbook of Elliptic and Hyperelliptic Curve Cryptography" page 363
    pub fn precomputed_table() -> (Vec<TauQuadratic>, Vec<TauExpansion<W>>) {
        let tau_w = Tau::<W>::pow();
        let mut rem_u = vec![];
        // let mut alpha_u = vec![];
        for u in (1..(1 << W)).step_by(2) {
            let rem = TauQuadratic::from(u).modulo(&tau_w);
            rem_u.push(rem);
        }
        todo!()
    }
}

pub type TauNAFw<const W: usize> = Vec<TauExpansion<W>>;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct TauQuadratic(pub IntegerQuadraticField);

impl Default for TauQuadratic {
    fn default() -> Self {
        TauLucasSequence::new().as_tau_quadratic()
    }
}

impl Add for TauQuadratic {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self(self.0 + other.0)
    }
}

impl Sub for TauQuadratic {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self(self.0 - other.0)
    }
}

impl Mul for TauQuadratic {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        Self(self.0 * other.0)
    }
}

impl Div for TauQuadratic {
    type Output = (Self, Self);
    fn div(self, other: Self) -> Self::Output {
        let (q, r) = self.0 / other.0;
        (Self(q), Self(r))
    }
}

impl Neg for TauQuadratic {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self(IntegerQuadraticField::new(-self.0.a0, -self.0.a1))
    }
}

impl From<IntegerBaseField> for TauQuadratic {
    fn from(value: IntegerBaseField) -> Self {
        Self(IntegerQuadraticField::new(value, 0))
    }
}

impl TauQuadratic {
    pub fn is_odd(&self) -> bool {
        self.0.a0.modulos(2) == 1
    }

    pub fn is_canonical(&self) -> bool {
        ((self.0.a0.abs() == 1) || (self.0.a0.abs() == 0))
            && ((self.0.a1.abs() == 1) || (self.0.a1.abs() == 0))
    }

    pub fn modulo(&self, modulus: &TauQuadratic) -> Self {
        let (_, r) = self.0 / modulus.0;
        Self(r)
    }

    pub fn one() -> Self {
        Self(IntegerQuadraticField::one())
    }

    pub fn zero() -> Self {
        Self(IntegerQuadraticField::zero())
    }

    pub fn from_tau() -> Self {
        Self(IntegerQuadraticField::new(0, 1))
    }

    pub fn new(n0: IntegerBaseField, n1: IntegerBaseField) -> Self {
        Self(IntegerQuadraticField::new(n0, n1))
    }

    pub fn value(&self) -> IntegerQuadraticField {
        self.0
    }

    pub fn pow(&self, exp: u32) -> Self {
        let mut result = self.clone();
        for _ in 0..(exp / 2) {
            result = result * result;
        }
        if exp % 2 == 1 {
            result = result * self.clone();
        }
        result
    }

    // convert to \tau-NAF representation
    // refer to "Handbook of Elliptic and Hyperelliptic Curve Cryptography", Algorithm 15.6
    pub fn to_naf(&self) -> Vec<i8> {
        let mut s = vec![];
        let (mut n0, mut n1) = (self.value().a0, self.value().a1);
        let mut r: i8;
        while n0.abs() + n1.abs() != 0 {
            if n0.modulos(2) == 1 {
                r = (2 - (n0 - 2 * n1).modulos(4)) as i8;
                n0 -= r as i64;
            } else {
                r = 0;
            }
            s.push(r);
            (n0, n1) = (n1 + MU * n0 / 2, -n0 / 2);
        }
        s
    }

    // refer to "Handbook of Elliptic and Hyperelliptic Curve Cryptography", Algorithm 15.17
    pub fn to_naf_w<const W: usize>(&self) -> TauNAFw<W> {
        // precomputed tables
        let alpha_table = vec![TauExpansion::default(); 1 << (W - 1)];
        let beta_table = vec![0; 1 << (W - 1)];
        let gamma_table = vec![0; 1 << (W - 1)];
        let hw = Tau::<W>::hw();
        let base_modulus = 1 << W;
        let mut result = vec![];
        let mut eta = self.clone();
        let mut r = TauExpansion::default();
        while eta.value().a0.abs() + eta.value().a1.abs() != 0 {
            if eta.value().a0.modulos(2) == 1 {
                // hw is approximate of tau mod 2^w
                let u = (eta.value().a0 + eta.value().a1 * hw) % base_modulus;
                let ui = u.abs() as usize;
                let delta = TauQuadratic::new(beta_table[ui], gamma_table[ui]);
                // after removing alpha_table[ui], eta is divisible by tau
                (eta, r) = if u > 0 {
                    (eta - delta, alpha_table[ui])
                } else {
                    (eta + delta, -alpha_table[ui])
                };
            } else {
                r = TauExpansion::default();
            }
            result.push(r);
            let (quo, rem) = eta / TauQuadratic::default();
            assert_eq!(rem, TauQuadratic::zero());
            eta = quo;
        }
        result
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct TauExpansion<const W: usize>(pub [IntegerBaseField; W]);

impl<const W: usize> Default for TauExpansion<W> {
    fn default() -> Self {
        TauExpansion([0; W])
    }
}

impl<const W: usize> Neg for TauExpansion<W> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        let mut arr = [0; W];
        for i in 0..W {
            arr[i] = -self.0[i];
        }
        TauExpansion(arr)
    }
}

impl<const W: usize> TauExpansion<W> {
    pub fn from_sparse(v: Vec<(usize, IntegerBaseField)>) -> Self {
        let mut arr = [0; W];
        for (i, coeff) in v.iter() {
            arr[*i] = *coeff;
        }
        TauExpansion(arr)
    }
}

impl<const W: usize> TauExpansion<W> {
    pub fn is_canonical(&self) -> bool {
        for i in 0..W {
            if (self.0[i].abs() != 1) && (self.0[i].abs() != 0) {
                return false;
            }
        }
        true
    }
}

impl<const W: usize> From<TauQuadratic> for TauExpansion<W> {
    fn from(v: TauQuadratic) -> Self {
        let mut arr = [0i64; W];
        arr[0] = v.0.a0;
        arr[1] = v.0.a1;
        return TauExpansion(arr);
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    // \tau^2 = MU * \tau - BIAS
    #[test]
    fn test_tau_pow() {
        assert_eq!(Tau::<2>::pow(), TauQuadratic::new(-BIAS * 1, MU))
    }

    // \tau^11 - 1 / \tau - 1 = 23 - 22 * \tau
    #[test]
    fn test_delta() {
        let nominator = Tau::<11>::pow() - TauQuadratic::one();
        let denominator = TauQuadratic::from_tau() - TauQuadratic::one();
        let (quotient, remainder) = nominator / denominator;
        assert_eq!(quotient, TauQuadratic::new(23, 0));
        assert_eq!(remainder, TauQuadratic::zero());
    }

    // refer to "Handbook of Elliptic and Hyperelliptic Curve Cryptography", Example 15.8
    #[test]
    fn test_to_tau_naf() {
        let scalar = TauQuadratic::from(409);
        let tau_naf = scalar.to_naf();
        let expected_tau_naf = vec![1, 0, 0, 1, 0, 0, 1, 0, -1, 0, 1, 0, 0, 0, 0, -1, 0, 0, -1];
        assert_eq!(tau_naf, expected_tau_naf);
    }

    // refer to "Handbook of Elliptic and Hyperelliptic Curve Cryptography" page 363
    // ring Z[\tau] / \tau^k is isomorphic to Z / 2^k, exists a map: \varphi(\tau) = h_k
    #[test]
    fn test_hw() {
        const K: usize = 5;
        let base_modulus = 1 << K;
        // hk = 2 * U_{k - 1} / U_k mod 2^k
        let mut l = TauLucasSequence::new();
        for i in 0..K - 1 {
            l = l.next();
            println!("U_{:?} = {:?}, U_{:?} = {:?}", i + 1, l.u0, i + 2, l.u1);
        }
        let uk_inv = l.u1.inv_mod_pow_k(K);
        let hk = (2 * l.u0 * uk_inv).modulos(base_modulus);

        // Verify tau^k ≡ 0 mod (tau^2 - hk * tau + 2)
        let tau_w = Tau::<K>::pow();
        assert_eq!(
            (tau_w.value().a0 + tau_w.value().a1 * hk).modulos(base_modulus),
            0
        );
        let (_, t) = TauQuadratic::from(15) / tau_w;
        println!("---- t = {:?} {:?}", t, t.to_naf());
    }
}
