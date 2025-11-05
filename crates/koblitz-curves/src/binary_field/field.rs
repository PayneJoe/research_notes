use std::fmt::Debug;
use std::ops::{Add, Mul, Neg, Shl, Sub};

pub const WORD_BITS: usize = 8;
pub type WORD = u8;
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct BigInt<const N: usize>(pub [WORD; N]);

impl<const N: usize> BigInt<N> {
    #[allow(dead_code)]
    pub fn to_bit_string(&self) -> String {
        self.0
            .iter()
            .map(|s| format!("{:08b}", s).chars().rev().collect::<String>())
            .collect::<String>()
            .trim_end_matches('0')
            .to_string()
    }

    // convert bit string to bigint, for example: "10011111,011" -> [249,6]
    #[allow(dead_code)]
    pub fn from_bit_string(s: &String) -> Self {
        assert!(s.len() < WORD_BITS * N);
        if s.len() == 0 {
            return Self::zero();
        }
        let mut result = Self::zero();
        let (mut w, mut w_mask) = (0 as WORD, 1 as WORD);
        let bytes = s.as_bytes();
        for i in 0..bytes.len() {
            if (i > 0) && (i % WORD_BITS == 0) {
                result.0[i / WORD_BITS - 1] = w;
                (w, w_mask) = (0 as WORD, 1 as WORD);
            }
            if bytes[i] == b'1' {
                w += w_mask;
            }
            w_mask <<= 1;
        }
        if bytes.len() % WORD_BITS != 0 {
            result.0[bytes.len() / WORD_BITS] = w;
        }
        result
    }

    pub fn zero() -> Self {
        Self([0 as WORD; N])
    }

    #[allow(dead_code)]
    pub fn one() -> Self {
        let mut result = Self::zero();
        result.0[0] = 1 as WORD;
        result
    }

    #[allow(dead_code)]
    pub fn is_zero(&self) -> bool {
        self.0 == [0 as WORD; N]
    }

    // modulus polynomial is assumed to be monic: X^N + h(x)
    // remove the leading one bit, so that we can obtain the residual polynomial h(x)
    #[allow(dead_code)]
    pub fn strip_leading_one(&self) -> Self {
        if self.is_zero() {
            return *self;
        }
        let mut shift = 0 as usize;
        for i in (0..N).rev() {
            let zeros = self.0[i].leading_zeros() as usize;
            if (shift == 0) && (zeros < 8) {
                shift += zeros;
                break;
            }
            shift += 8;
        }
        *self << shift
    }
}

impl<const N: usize> Add for BigInt<N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = [0 as WORD; N];
        for i in 0..N {
            result[i] = self.0[i] ^ rhs.0[i];
        }
        BigInt(result)
    }
}

// algorithm 2.34 in "Guide to Elliptic Curve Cryptography"
impl<const N: usize> Mul for BigInt<N> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut c = Self::zero();
        let mut word_mask = 1 as WORD;
        let mut right = rhs;
        for k in 0..WORD_BITS {
            for j in 0..N {
                if self.0[j] & word_mask == 1 {
                    c = c + right << j * WORD_BITS;
                }
            }
            if k != WORD_BITS - 1 {
                right = right << 1;
            }
            word_mask <<= 1;
        }
        c
    }
}

impl<const N: usize> Shl<usize> for BigInt<N> {
    type Output = Self;

    fn shl(self, shift: usize) -> Self::Output {
        let mut result = [0 as WORD; N];
        let byte_shift = shift / 8;
        let bit_shift = shift % 8;

        for i in (0..N).rev() {
            if i >= byte_shift {
                result[i] = self.0[i - byte_shift] << bit_shift;
                if bit_shift > 0 && i - byte_shift > 0 {
                    result[i] |= self.0[i - byte_shift - 1] >> (8 - bit_shift);
                }
            }
        }
        BigInt(result)
    }
}

pub trait BinaryField<const N: usize>:
    Debug
    + Eq
    + PartialEq
    + Copy
    + Clone
    + Add<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Mul<Self, Output = Self>
    + Neg<Output = Self>
{
    const MODULUS: BigInt<N>;
    fn reduce(element: BigInt<N>) -> Self;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_from_bit_string() {
        let s = String::from_str("10011111011").unwrap();
        let n = BigInt::<4>::from_bit_string(&s);
        let expect = BigInt([249, 6, 0, 0]);
        assert_eq!(n, expect, "Test for BigInt::from_bit_string failed!");
        assert_eq!(
            n.to_bit_string(),
            s,
            "Test for BigInt::to_bit_string failed!"
        );
    }
}
