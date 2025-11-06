use crate::binary_field::field::{BigInt, BigInt2, BinaryField, WORD_SIZE};
use core::ops::{Add, Mul, Neg, Shl, Sub};

// binary field Fq = GF(2^m) / f(X), where m = 163
// N = 21 when word = u8
pub const M: usize = 163;
pub const N: usize = 21;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Fq(BigInt<N>);

#[allow(dead_code)]
impl Fq {
    pub fn zero() -> Self {
        Self(BigInt::<N>::zero())
    }

    pub fn one() -> Self {
        Self(BigInt::<N>::one())
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
    const R: BigInt<N> = BigInt([
        201, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ]);
    const UK: [BigInt<N>; WORD_SIZE] = [
        BigInt([
            201, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ]),
        BigInt([
            146, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ]),
        BigInt([
            36, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ]),
        BigInt([
            72, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ]),
        BigInt([
            144, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ]),
        BigInt([
            32, 25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ]),
        BigInt([
            64, 50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ]),
        BigInt([
            128, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ]),
    ];

    // Algorithm 2.40 in "Guide to Elliptic Curve Cryptography"
    fn reduce(ele: BigInt2<N>) -> Self {
        todo!()
    }
}
