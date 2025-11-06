use crate::binary_field::field::{BigInt, BinaryField, WORD, WORD_SIZE};
use core::ops::{Add, Mul, Neg, Shl, Sub};

// Fq = GF(2^m)
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Fq<const N: usize>(BigInt<N>);

impl<const N: usize> Fq<N> {
    pub fn zero() -> Self {
        Self(BigInt([0 as WORD; N]))
    }

    pub fn one() -> Self {
        let mut one = [0 as WORD; N];
        one[N - 1] = 1 as WORD;
        Self(BigInt(one))
    }
}

impl<const N: usize> Add for Fq<N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl<const N: usize> Shl<usize> for Fq<N> {
    type Output = Self;

    fn shl(self, shift: usize) -> Self::Output {
        Self(self.0 << shift)
    }
}

impl<const N: usize> Mul for Fq<N> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

// impl<const N: usize> BinaryField<N> for Fq<N> {
//     const MODULUS: INTEGER<N> = [0; N];
//     fn reduce(element: INTEGER<N>) -> Self {
//         Fq
//     }
// }
