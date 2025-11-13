#![allow(unused_imports)]
pub mod r_tau;
pub mod z_tau;

use self::r_tau::{R, RTau};
use self::z_tau::{Z, ZTau};

// N(x) = \prod_i x * x_i, where x_i are all the conjugates of x
// For example, if x is a complex number, then there are only two conjugative numbers
pub trait Norm<T> {
    fn norm(&self) -> T;
}

// Characteristic polynomial of Frobenius Endomorphism for binary curve K-233: \tau^2 - \mu * \tau + 2
// \mu = (-1)^{1 - a_2}, \mu = -1 when a_2 = 0 in K-233 curve
pub trait Tau: Sized {
    const MU: Z = Z(-1);
    const CHAR: Z = Z(2);
    fn one() -> Self;
    fn zero() -> Self;
    // map to its conjugative representation, for example, a + b * \bar{\tau} = (a + b * \mu) - b * \tau, and vice veras
    fn automorphism(&self) -> Self;
    // Z[\tau] -> Z[\bar{tau}] -> Z[\tau]
    // conjugative object, i.e. a + b * \bar{\tau} -> a + b * \tau, which is can map to Self with automorphism
    fn conjugate(&self) -> Self {
        Self::automorphism(self)
    }
    //////////////////////////////////////// For Window-based Tau-adic Expansion
    // \tau^w = U_w * \tau - U_{w - 1} * BIAS, where w is the window size
    fn pow(w: usize) -> ZTau {
        if w == 0 {
            return ZTau::one();
        }
        if w == 1 {
            return ZTau::default();
        }
        let lucas_seq = LucasSequence::new(Z(0), Z(1)).n_steps(w - 1);
        ZTau::from(&lucas_seq)
    }
    // hw = 2 * U_{w - 1} * U_w^{-1}, hw is in the kernel of map \phi_w: Z[\tau] -> Z[2^w], i.e. \phi_w(h_w) = 0 (mod 2^w)
    // for example, regarding a + b * \tau + c * \tau^2 + ... \in Z[\tau], we can map it to a + b * hw + c * hw^2 + ... (mod 2^w) = 0
    fn h_w(w: usize) -> Z {
        let modulus = Z(1 << w);
        let lucas_seq = LucasSequence::new(Z(0), Z(1)).n_steps(w - 1);
        let u_w_minus_1 = lucas_seq.u0;
        let u_w_inv = lucas_seq.u1.inv_mod(modulus);
        (Z(2) * u_w_minus_1 * u_w_inv).reduce(modulus)
    }
    // u (mod \tau^w), \alpha_u = tauNAF(u mod \tau^w)
    fn precomputed_table(w: usize) -> (Vec<ZTau>, Vec<Vec<Z>>) {
        let tau_w = Self::pow(w);
        let u_mod_tau_w = (1..(1 << (w - 1)))
            .step_by(2)
            .map(|u| ZTau::new(Z(u as i64), Z(0)).reduce(tau_w))
            .collect::<Vec<_>>();
        let alpha_u = u_mod_tau_w.iter().map(|v| v.tauNAF()).collect::<Vec<_>>();
        (u_mod_tau_w, alpha_u)
    }
}

// refer to "Handbook of Elliptic and Hyperelliptic Curve Cryptography", equation 15.4
#[derive(Copy, Clone, Debug)]
pub struct LucasSequence {
    u0: Z,
    u1: Z,
}
impl LucasSequence {
    pub fn new(u0: Z, u1: Z) -> Self {
        Self { u0, u1 }
    }
    fn next(&self) -> Self {
        Self {
            u0: self.u1,
            u1: ZTau::MU * self.u1 - ZTau::CHAR * self.u0,
        }
    }
    pub fn n_steps(&self, n: usize) -> Self {
        let mut result = self.clone();
        for _ in 0..n {
            result = result.next();
        }
        result
    }
}
