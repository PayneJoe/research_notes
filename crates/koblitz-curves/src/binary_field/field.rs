use std::fmt::Debug;
use std::ops::{Add, Mul, Neg, Shl, Sub};

// u8 word only for testing purpose, actually we will use u32 or u64 as one word
pub type WORD = u8;
pub const WORD_SIZE: usize = 8;
// pub type WORD = u32;
// pub const WORD_SIZE: usize = 32;
// window size for caching when doing bigint multiplication
pub const WINDOW_SIZE: usize = 4;

#[allow(dead_code)]
pub trait PolynomialSquaring: Sized {
    fn squaring(&self) -> [Self; 2];
}

impl PolynomialSquaring for u8 {
    // squaring a byte would blow up two times of its capacity
    fn squaring(&self) -> [Self; 2] {
        let mut result = [0 as u8; 2];
        // byte to bits
        let byte_bits = (0..8).map(|i| (self >> i) & 1 == 1).collect::<Vec<bool>>();
        // insert zeros in lower byte
        for i in 0..4 {
            if byte_bits[i] {
                result[0] += 1 << (2 * i);
            }
        }
        // insert zeros in higher word
        for i in 0..4 {
            if byte_bits[4 + i] {
                result[1] += 1 << (2 * i);
            }
        }
        result
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct BigInt<const N: usize>(pub [WORD; N]);

#[allow(dead_code)]
impl<const N: usize> BigInt<N> {
    // convert bigint to bit string, for example: [249, 6] -> "10011111,011" when word = u8
    pub fn to_bit_string(&self) -> String {
        self.0
            .iter()
            .map(|s| format!("{:08b}", s).chars().rev().collect::<String>())
            .collect::<String>()
            .trim_end_matches('0')
            .to_string()
    }

    // convert bit string with bit-wise big-ending ordering (bits from left to right) to bigint, for example: "10011111,011" -> [249,6] when word = u8
    pub fn from_bit_string(s: &String) -> Self {
        assert!(s.len() <= WORD_SIZE * N);
        if s.len() == 0 {
            return Self::zero();
        }
        let mut result = Self::zero();
        let (mut w, mut w_mask) = (0 as WORD, 1 as WORD);
        let bytes = s.as_bytes();
        for i in 0..bytes.len() {
            if (i > 0) && (i % WORD_SIZE == 0) {
                result.0[i / WORD_SIZE - 1] = w;
                (w, w_mask) = (0 as WORD, 1 as WORD);
            }
            if bytes[i] == b'1' {
                w += w_mask;
            }
            w_mask <<= 1;
        }
        result.0[(bytes.len() - 1) / WORD_SIZE] = w;
        result
    }

    // modulus polynomial is assumed to be monic: X^N + h(x)
    // remove the leading one bit, so that we can obtain the residual polynomial h(x)
    pub fn strip_leading_one(&self) -> Self {
        if self.is_zero() {
            return *self;
        }
        let mut shift = 0 as usize;
        for i in (0..N).rev() {
            let zeros = self.0[i].leading_zeros() as usize;
            if (shift == 0) && (zeros < WORD_SIZE) {
                shift += zeros;
                break;
            }
            shift += WORD_SIZE;
        }
        *self << shift
    }
    pub fn zero() -> Self {
        Self([0 as WORD; N])
    }

    pub fn one() -> Self {
        let mut result = Self::zero();
        result.0[0] = 1 as WORD;
        result
    }

    pub fn is_zero(&self) -> bool {
        self.0 == [0 as WORD; N]
    }

    // Algorithm 2.39 in "Gude to Elliptic Curve Cryptography"
    pub fn squaring(&self) -> BigInt2<N> {
        let mut result = Vec::with_capacity(2 * N);
        // precomputation for byte squaring
        let capacity = 1 << 8;
        let mut lookup_table = Vec::with_capacity(capacity);
        lookup_table.push([0u8; 2]);
        for v in 1..capacity {
            lookup_table.push((v as u8).squaring());
        }
        // insert zeros by byte
        for i in 0..N {
            // multiple bytes in a word
            let word_bytes = self.0[i].to_be_bytes();
            for byte in word_bytes {
                let byte_squaring = lookup_table[byte as usize];
                result.push(byte_squaring[0]);
                result.push(byte_squaring[1]);
            }
        }
        let words = result
            .chunks(WORD_SIZE / 8)
            .map(|v| WORD::from_be_bytes(v.try_into().unwrap()))
            .collect::<Vec<_>>();
        BigInt2([
            Self(words[..N].try_into().unwrap()),
            Self(words[N..].try_into().unwrap()),
        ])
    }

    pub fn shr_words(&self, n: usize) -> (Self, Self) {
        assert!(n <= N);
        (
            Self(self.0[n..].try_into().unwrap()),
            Self(self.0[..n].try_into().unwrap()),
        )
    }

    pub fn shl_word(&self, n: usize) -> (Self, Self) {
        assert!(n <= N);
        (
            Self(self.0[0..(N - n)].try_into().unwrap()),
            Self(self.0[(N - n)..].try_into().unwrap()),
        )
    }
}

impl<const N: usize> From<WORD> for BigInt<N> {
    fn from(w: WORD) -> Self {
        let mut result = Self::zero();
        result.0[0] = w;
        result
    }
}

impl<const N: usize> From<Vec<WORD>> for BigInt<N> {
    fn from(v: Vec<WORD>) -> Self {
        if v.len() >= N {
            Self(v[..N].try_into().unwrap())
        } else {
            let mut result = [0 as WORD; N];
            result.copy_from_slice(&v);
            Self(result)
        }
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
    type Output = BigInt2<N>;

    // TODO: add a small lookup table would be a little bit helpful
    fn mul(self, rhs: WORD) -> Self::Output {
        let mut c = BigInt2::<N>::zero();
        let mut word_mask = 1 as WORD;
        let mut left = self;
        for j in 0..WORD_SIZE {
            if (rhs & word_mask) == word_mask {
                c = c + left;
            }
            if j != WORD_SIZE - 1 {
                left = left << 1;
            }
            word_mask <<= 1;
        }
        c
    }
}

// Algorithm 11.37 in "Handbook of Elliptic and HyperElliptic Curve Cryptography"
// This is a window-based optimized algorithm of binary polynomial multiplication,
impl<const N: usize> Mul for BigInt<N> {
    type Output = BigInt2<N>;

    fn mul(self, rhs: Self) -> Self::Output {
        // cache lookup table
        assert!(WORD_SIZE % WINDOW_SIZE == 0);
        let capacity = 1 << WINDOW_SIZE;
        let mut lookup_table = vec![BigInt2::<N>::zero(); capacity];
        for i in 1..capacity {
            if i % 2 == 0 {
                lookup_table[i] = lookup_table[i / 2] << 1;
            } else {
                lookup_table[i] = lookup_table[i - 1] + rhs;
            }
        }
        // iterate by window
        let mut c = BigInt2::<N>::zero();
        for j in (0..(WORD_SIZE / WINDOW_SIZE)).rev() {
            for i in 0..N {
                let chunk_word = (self.0[i] >> (j * WINDOW_SIZE)) & ((capacity - 1) as WORD);
                c = c + (lookup_table[chunk_word as usize] << (i * WORD_SIZE));
            }
            if j != 0 {
                c = c << WINDOW_SIZE;
            }
        }
        c
    }
}

impl<const N: usize> Shl<usize> for BigInt<N> {
    type Output = Self;

    fn shl(self, shift: usize) -> Self::Output {
        let mut result = [0 as WORD; N];
        let word_shift = shift / WORD_SIZE;
        let bit_shift = shift % WORD_SIZE;

        for i in (0..N).rev() {
            if i >= word_shift {
                result[i] = self.0[i - word_shift] << bit_shift;
                if bit_shift > 0 && i - word_shift > 0 {
                    result[i] |= self.0[i - word_shift - 1] >> (WORD_SIZE - bit_shift);
                }
            }
        }
        BigInt(result)
    }
}

impl<const N: usize> Neg for BigInt<N> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self
    }
}

impl<const N: usize> Sub<Self> for BigInt<N> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + rhs
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct BigInt2<const N: usize>(pub [BigInt<N>; 2]);

impl<const N: usize> BigInt2<N> {
    pub fn zero() -> Self {
        Self([BigInt::<N>::zero(), BigInt::<N>::zero()])
    }
}

impl<const N: usize> Add<Self> for BigInt2<N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self([self.0[0] + rhs.0[0], self.0[1] + rhs.0[1]])
    }
}

impl<const N: usize> Add<BigInt<N>> for BigInt2<N> {
    type Output = Self;

    fn add(self, rhs: BigInt<N>) -> Self::Output {
        Self([self.0[0] + rhs, self.0[1]])
    }
}

impl<const N: usize> Shl<usize> for BigInt2<N> {
    type Output = Self;

    fn shl(self, shift: usize) -> Self::Output {
        let mut result = vec![0 as WORD; 2 * N];
        let word_shift = shift / WORD_SIZE;
        let bit_shift = shift % WORD_SIZE;

        let mut words = self.0[0].0.to_vec();
        words.extend_from_slice(&self.0[1].0);

        for i in (0..(2 * N)).rev() {
            if i >= word_shift {
                result[i] = words[i - word_shift] << bit_shift;
                if bit_shift > 0 && i - word_shift > 0 {
                    result[i] |= words[i - word_shift - 1] >> (WORD_SIZE - bit_shift);
                }
            }
        }
        Self([
            BigInt(result[..N].try_into().unwrap()),
            BigInt(result[N..].try_into().unwrap()),
        ])
    }
}

#[allow(dead_code)]
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
    // irreducible binary polynomial: f(X) = X^M + R(X) where M is the degree of binary polynomial, and R(X) is residual polynomial
    // which M < N * WORD_SIZE, and deg(R) < M
    const M: usize;
    const R: BigInt<N>;
    const UK: [BigInt<N>; WORD_SIZE];
    fn reduce(element: BigInt2<N>) -> Self;
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
            String::from_str("").unwrap(),
        )];
        for (u_bit_string, v_bit_string, w_low_bit_string, w_high_bit_string) in test_data {
            let (u, v, w_expected) = (
                BigInt::<4>::from_bit_string(&u_bit_string),
                BigInt::<4>::from_bit_string(&v_bit_string),
                BigInt2([
                    BigInt::<4>::from_bit_string(&w_low_bit_string),
                    BigInt::<4>::from_bit_string(&w_high_bit_string),
                ]),
            );
            let w = u * v;
            assert_eq!(w, w_expected, "Test for BigInt::mul failed!");
            assert!(
                (w.0[0].to_bit_string() == w_low_bit_string)
                    && (w.0[1].to_bit_string() == w_high_bit_string)
            );
        }
    }

    // squaring over a byte
    #[test]
    fn test_u8_squaring() {
        let test_data = [(4u8, [16u8, 0u8]), (3u8, [5u8, 0u8]), (149u8, [17u8, 65u8])];
        for (v, v_squaring_expected) in test_data {
            let v_squaring = v.squaring();
            assert_eq!(v_squaring, v_squaring_expected);
        }
    }

    // squaring over a bigint
    #[test]
    fn test_bigint_squaring() {
        let test_data = [
            (
                String::from_str("10011111011").unwrap(),
                [
                    String::from_str("1000001010101010").unwrap(),
                    String::from_str("0010100000000000").unwrap(),
                ],
            ),
            (
                String::from_str("1001111100100101").unwrap(),
                [
                    String::from_str("1000001010101010").unwrap(),
                    String::from_str("0000100000100010").unwrap(),
                ],
            ),
        ];
        for (v_bit_string, v_squaring_bit_string) in test_data {
            let (v, v_squaring_expected) = (
                BigInt::<2>::from_bit_string(&v_bit_string),
                BigInt2([
                    BigInt::<2>::from_bit_string(&v_squaring_bit_string[0]),
                    BigInt::<2>::from_bit_string(&v_squaring_bit_string[1]),
                ]),
            );
            let v_squaring = v.squaring();
            assert_eq!(
                v_squaring, v_squaring_expected,
                "Test for BigInt::squaring failed!"
            );
        }
    }
}
