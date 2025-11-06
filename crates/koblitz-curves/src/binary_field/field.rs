use std::fmt::Debug;
use std::ops::{Add, Mul, Neg, Shl, Sub};

// u8 word only for testing purpose, actually we use u32 or u64
pub const WORD_BITS: usize = 8;
pub type WORD = u8;
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct BigInt<const N: usize>(pub [WORD; N]);

impl<const N: usize> BigInt<N> {
    // convert bigint to bit string, for example: [249, 6] -> "10011111,011"
    #[allow(dead_code)]
    pub fn to_bit_string(&self) -> String {
        self.0
            .iter()
            .map(|s| format!("{:08b}", s).chars().rev().collect::<String>())
            .collect::<String>()
            .trim_end_matches('0')
            .to_string()
    }

    // convert bit string with bit-wise big-ending ordering (bits from left to right) to bigint, for example: "10011111,011" -> [249,6]
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
        result.0[(bytes.len() - 1) / WORD_BITS] = w;
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

// Algorithm 11.34 in "Handbook of Elliptic and HyperElliptic Curve Cryptography"
// This is a naive algorithm of binary polynomial multiplication over a word
impl<const N: usize> Mul<WORD> for BigInt<N> {
    type Output = Self;

    fn mul(self, rhs: WORD) -> Self::Output {
        let mut c = Self::zero();
        let mut word_mask = 1 as WORD;
        let mut left = self;
        for j in 0..WORD_BITS {
            if (rhs & word_mask) == word_mask {
                c = c + left;
            }
            if j != WORD_BITS - 1 {
                left = left << 1;
            }
            word_mask <<= 1;
        }
        c
    }
}

// Algorithm 11.37 in "Handbook of Elliptic and HyperElliptic Curve Cryptography"
// This is a window-based optimized algorithm of binary polynomial multiplication,
// set a small window size `k = 2`
impl<const N: usize> Mul for BigInt<N> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        // cache lookup table
        let k = 2;
        assert!(WORD_BITS % k == 0);
        let capacity = 1 << k;
        let mut lookup_table = Vec::with_capacity(capacity);
        lookup_table.push(Self::zero());
        for i in 1..capacity {
            if i % 2 == 0 {
                lookup_table.push(lookup_table[i / 2] << 1);
            }
            lookup_table.push(*lookup_table.last().unwrap() + rhs);
        }
        // iterate by window
        let mut c = Self::zero();
        for j in (0..(WORD_BITS / k)).rev() {
            for i in 0..N {
                let chunk_word = (self.0[i] >> (j * k)) & ((capacity - 1) as WORD);
                c = c + (lookup_table[chunk_word as usize] << (i * WORD_BITS));
            }
            if j != 0 {
                c = c << k;
            }
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

    // test bit string with big ending (high bit in the end) conversion from/to bigint
    #[test]
    fn test_bit_string() {
        let test_data = [(
            String::from_str("10011111011").unwrap(),
            BigInt([249, 6, 0, 0]),
        )];
        for (v_bit_string, v_expected) in test_data {
            let v = BigInt::<4>::from_bit_string(&v_bit_string);
            assert_eq!(v, v_expected, "Test for BigInt::from_bit_string failed!");
            assert_eq!(
                v.to_bit_string(),
                v_bit_string,
                "Test for BigInt::to_bit_string failed!"
            );
        }
    }

    // Example 11.36 in "Handbook of Elliptic and HyperElliptic Curve Cryptography"
    // u(X) = X^5 + X^4 + X^2 + X, v(X) = X^10 + X^9 + X^7 + X^6 + X^5 + X^4 + X^3 + 1
    // w(X) = u(X) * v(X) = X^15 + X^13 + X^10 + X^9 + X^7 + X^5 + X^2 + X
    #[test]
    fn test_bigint_mul() {
        let test_data = vec![(
            String::from_str("011011").unwrap(),
            String::from_str("10011111011").unwrap(),
            String::from_str("0110010101100101").unwrap(),
        )];
        for (u_bit_string, v_bit_string, w_bit_string) in test_data {
            let (u, v, w_expected) = (
                BigInt::<4>::from_bit_string(&u_bit_string),
                BigInt::<4>::from_bit_string(&v_bit_string),
                BigInt::<4>::from_bit_string(&w_bit_string),
            );
            let w = u * v;
            assert_eq!(w, w_expected, "Test for BigInt::mul failed!");
            assert_eq!(w.to_bit_string(), w_bit_string);
        }
    }
}
