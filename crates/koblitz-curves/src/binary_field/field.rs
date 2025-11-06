use crate::binary_field::polynomial::{
    BinaryField, BinaryPolynomial, BinaryPolynomial2, WORD_SIZE,
};
use core::ops::{Add, Mul, Neg, Shl, Sub};

// binary field Fq = GF(2^m) / f(X), where m = 163
// N = 21 when word = u8
pub const M: usize = 163;
pub const N: usize = 21;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Fq(BinaryPolynomial<N>);

#[allow(dead_code)]
impl Fq {
    pub fn zero() -> Self {
        Self(BinaryPolynomial::<N>::zero())
    }

    pub fn one() -> Self {
        Self(BinaryPolynomial::<N>::one())
    }
}

impl Add for Fq {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Shl<usize> for Fq {
    type Output = Self;

    fn shl(self, shift: usize) -> Self::Output {
        Self(self.0 << shift)
    }
}

impl Mul for Fq {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::reduce(self.0 * rhs.0)
    }
}

impl Neg for Fq {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl Sub<Self> for Fq {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl BinaryField<N> for Fq {
    const M: usize = M;
    const R: BinaryPolynomial<N> = BinaryPolynomial([
        201, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ]);
    const UK: [BinaryPolynomial<N>; WORD_SIZE] = [
        BinaryPolynomial([
            201, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ]),
        BinaryPolynomial([
            146, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ]),
        BinaryPolynomial([
            36, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ]),
        BinaryPolynomial([
            72, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ]),
        BinaryPolynomial([
            144, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ]),
        BinaryPolynomial([
            32, 25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ]),
        BinaryPolynomial([
            64, 50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ]),
        BinaryPolynomial([
            128, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ]),
    ];

    // Algorithm 2.40 in "Guide to Elliptic Curve Cryptography"
    fn reduce(ele: BinaryPolynomial2<N>) -> Self {
        let mut c = BinaryPolynomial::<N>::zero();
        for i in ((N * WORD_SIZE)..(2 * N * WORD_SIZE - 1)).rev() {
            let j = (i - N * WORD_SIZE) / WORD_SIZE;
            let k = (i - N * WORD_SIZE) - j * WORD_SIZE;
            let word_mask = 1 << k;
            if ele.0[1].at(j).unwrap() & word_mask == word_mask {
                c = c.trunc_add(j, Self::UK[k]);
            }
        }
        Self(c)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_fq_reduce() {
        todo!()
    }
}
