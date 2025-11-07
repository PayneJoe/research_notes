use hex;
use std::fmt::Debug;
use std::ops::{Add, Mul, Neg, Shl, Shr, Sub};

// u8 word only for testing purpose, actually we will use u32 or u64 as one word
pub type WORD = u8;
pub const WORD_SIZE: usize = 8;
// pub type WORD = u32;
// pub const WORD_SIZE: usize = 32;
// window size for caching when doing bigint multiplication
pub const WINDOW_SIZE: usize = 4;

#[allow(dead_code)]
pub trait BinaryPolynomialSquaring: Sized {
    fn squaring(&self) -> [Self; 2];
}

impl BinaryPolynomialSquaring for u8 {
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

// binary polynomial representation for bigint
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct BinaryPolynomial<const N: usize>(pub [WORD; N]);

#[allow(dead_code)]
impl<const N: usize> BinaryPolynomial<N> {
    pub fn at(&self, index: usize) -> Option<&WORD> {
        if index < N {
            Some(&self.0[index])
        } else {
            None
        }
    }

    // to little ending hex bytes string
    pub fn to_hex_string(&self) -> String {
        let bytes = self
            .0
            .iter()
            .map(|w| w.to_le_bytes())
            .rev()
            .collect::<Vec<_>>()
            .concat();
        format!("0x{}", hex::encode(&bytes))
    }

    // from little ending hex bytes string
    pub fn from_hex_string(s: &String) -> Self {
        assert!(s.starts_with("0x"));
        let mut s_trimed = s.strip_prefix("0x").unwrap().to_string();
        if s_trimed.len() % 2 == 1 {
            s_trimed = format!("0{}", s_trimed);
        }
        let bytes = hex::decode(s_trimed)
            .expect("Invalid hex string")
            .into_iter()
            .rev()
            .collect::<Vec<_>>();
        let words = bytes
            .chunks(WORD_SIZE / 8)
            .map(|v| WORD::from_be_bytes(v.try_into().unwrap()))
            .collect::<Vec<_>>();
        Self::from(words)
    }

    // to little ending bit string
    pub fn to_bit_string(&self) -> String {
        let bit_string = self
            .0
            .iter()
            .map(|w| {
                w.to_le_bytes()
                    .iter()
                    .map(|b| format!("{:08b}", b).chars().collect::<String>())
                    .collect::<String>()
            })
            .rev()
            .collect::<String>();
        format!("0b{}", bit_string.trim_start_matches("0"))
    }

    // from little ending bit string
    pub fn from_bit_string(s: &String) -> Self {
        assert!(s.starts_with("0b"));
        let bit_string = s.strip_prefix("0b").unwrap();
        assert!(bit_string.len() <= WORD_SIZE * N);
        if bit_string.len() == 0 {
            return Self::zero();
        }
        let mut result = Self::zero();
        let (mut w, mut w_mask) = (0 as WORD, 1 as WORD);
        let bit_bytes = bit_string.as_bytes().into_iter().rev().collect::<Vec<_>>();
        for i in 0..bit_bytes.len() {
            if (i > 0) && (i % WORD_SIZE == 0) {
                result.0[i / WORD_SIZE - 1] = w;
                (w, w_mask) = (0 as WORD, 1 as WORD);
            }
            if *bit_bytes[i] == b'1' {
                w += w_mask;
            }
            w_mask <<= 1;
        }
        result.0[(bit_bytes.len() - 1) / WORD_SIZE] = w;
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
    pub fn squaring(&self) -> BinaryPolynomial2<N> {
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
        BinaryPolynomial2([
            Self::from(words[..N].to_vec()),
            Self::from(words[N..].to_vec()),
        ])
    }

    pub fn trunc_add(&self, trunc_index: usize, rhs: Self) -> Self {
        let (truncated, low) = self.shr_words(trunc_index);
        let mut result = [0 as WORD; N];
        let (high, _) = (truncated + rhs).shl_words(trunc_index);
        result[..trunc_index].copy_from_slice(&low.0[..trunc_index]);
        result[trunc_index..].copy_from_slice(&high.0[trunc_index..]);
        Self(result)
    }

    pub fn shr_words(&self, n: usize) -> (Self, Self) {
        assert!(n < N);
        (
            Self::from(self.0[n..].to_vec()),
            Self::from(self.0[..n].to_vec()),
        )
    }

    pub fn shl_words(&self, n: usize) -> (Self, Self) {
        assert!(n < N);
        let mut left = BinaryPolynomial::<N>::zero();
        left.0[n..].copy_from_slice(&self.0[..(N - n)]);
        let right = Self::from(self.0[(N - 1 - n)..].to_vec());
        (left, right)
    }
}

impl<const N: usize> From<WORD> for BinaryPolynomial<N> {
    fn from(w: WORD) -> Self {
        let mut result = Self::zero();
        result.0[0] = w;
        result
    }
}

impl<const N: usize> From<Vec<WORD>> for BinaryPolynomial<N> {
    fn from(v: Vec<WORD>) -> Self {
        if v.len() >= N {
            Self(v[..N].try_into().unwrap())
        } else {
            let mut result = [0 as WORD; N];
            result[..v.len()].copy_from_slice(&v);
            Self(result)
        }
    }
}

impl<const N: usize> Add for BinaryPolynomial<N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = [0 as WORD; N];
        for i in 0..N {
            result[i] = self.0[i] ^ rhs.0[i];
        }
        BinaryPolynomial(result)
    }
}

// Algorithm 11.34 in "Handbook of Elliptic and HyperElliptic Curve Cryptography"
// This is a naive algorithm of binary polynomial multiplication over a word
impl<const N: usize> Mul<WORD> for BinaryPolynomial<N> {
    type Output = BinaryPolynomial2<N>;

    // TODO: add a small lookup table would be a little bit helpful
    fn mul(self, rhs: WORD) -> Self::Output {
        let mut c = BinaryPolynomial2::<N>::zero();
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
impl<const N: usize> Mul for BinaryPolynomial<N> {
    type Output = BinaryPolynomial2<N>;

    fn mul(self, rhs: Self) -> Self::Output {
        // cache lookup table
        assert!(WORD_SIZE % WINDOW_SIZE == 0);
        let capacity = 1 << WINDOW_SIZE;
        let mut lookup_table = vec![BinaryPolynomial2::<N>::zero(); capacity];
        for i in 1..capacity {
            if i % 2 == 0 {
                lookup_table[i] = lookup_table[i / 2] << 1;
            } else {
                lookup_table[i] = lookup_table[i - 1] + rhs;
            }
        }
        // iterate by window
        let mut c = BinaryPolynomial2::<N>::zero();
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

impl<const N: usize> Shl<usize> for BinaryPolynomial<N> {
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
        BinaryPolynomial(result)
    }
}

impl<const N: usize> Shr<usize> for BinaryPolynomial<N> {
    type Output = Self;

    fn shr(self, shift: usize) -> Self::Output {
        let mut result = [0 as WORD; N];
        let word_shift = shift / WORD_SIZE;
        let bit_shift = shift % WORD_SIZE;

        for i in 0..N {
            if i >= word_shift {
                result[N - 1 - i] = self.0[N - 1 - i + word_shift] >> bit_shift;
                if bit_shift > 0 && i - word_shift > 0 {
                    result[N - 1 - i] |= self.0[N - i + word_shift] << (WORD_SIZE - bit_shift);
                }
            }
        }
        BinaryPolynomial(result)
    }
}

impl<const N: usize> Neg for BinaryPolynomial<N> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self
    }
}

impl<const N: usize> Sub<Self> for BinaryPolynomial<N> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + rhs
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct BinaryPolynomial2<const N: usize>(pub [BinaryPolynomial<N>; 2]);

#[allow(dead_code)]
impl<const N: usize> BinaryPolynomial2<N> {
    // split two binary polynomial at a index within the lower one
    pub fn split_at(&self, index: usize) -> [BinaryPolynomial<N>; 2] {
        assert!(index < N * WORD_SIZE);

        let mut result = [BinaryPolynomial::<N>::zero(); 2];
        result[0] = (self.0[0] << (N * WORD_SIZE - index)) >> (N * WORD_SIZE - index);
        result[1] = (self.0[1] << (N * WORD_SIZE - index)) + (self.0[0] >> index);
        result
    }
}

impl<const N: usize> From<BinaryPolynomial<N>> for BinaryPolynomial2<N> {
    fn from(v: BinaryPolynomial<N>) -> Self {
        let mut result = Self::zero();
        result.0[0] = v;
        result
    }
}

impl<const N: usize> BinaryPolynomial2<N> {
    pub fn zero() -> Self {
        Self([BinaryPolynomial::<N>::zero(), BinaryPolynomial::<N>::zero()])
    }
}

impl<const N: usize> Add<Self> for BinaryPolynomial2<N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self([self.0[0] + rhs.0[0], self.0[1] + rhs.0[1]])
    }
}

impl<const N: usize> Add<BinaryPolynomial<N>> for BinaryPolynomial2<N> {
    type Output = Self;

    fn add(self, rhs: BinaryPolynomial<N>) -> Self::Output {
        Self([self.0[0] + rhs, self.0[1]])
    }
}

impl<const N: usize> Shl<usize> for BinaryPolynomial2<N> {
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
            BinaryPolynomial(result[..N].try_into().unwrap()),
            BinaryPolynomial(result[N..].try_into().unwrap()),
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
    // which M <= N * WORD_SIZE, and deg(R) < M
    const M: usize;
    const R: BinaryPolynomial<N>;
    const UK: [BinaryPolynomial<N>; WORD_SIZE];
    fn reduce(element: BinaryPolynomial2<N>) -> Self;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_hex_string() {
        let test_data = [(
            String::from_str("0x06f9").unwrap(),
            BinaryPolynomial([249, 6, 0, 0]),
        )];
        for (v_hex_string, v_expected) in test_data {
            let v = BinaryPolynomial::<4>::from_hex_string(&v_hex_string);
            assert_eq!(v, v_expected);
        }
    }

    // test bit string with big ending (high bit in the end) conversion from/to bigint
    #[test]
    fn test_bit_string() {
        let test_data = [(
            String::from_str("0b11011111001").unwrap(),
            BinaryPolynomial([249, 6, 0, 0]),
        )];
        for (v_bit_string, v_expected) in test_data {
            let v = BinaryPolynomial::<4>::from_bit_string(&v_bit_string);
            assert_eq!(
                v, v_expected,
                "Test for BinaryPolynomial::from_bit_string failed!"
            );
            assert_eq!(
                v.to_bit_string(),
                v_bit_string,
                "Test for BinaryPolynomial::to_bit_string failed!"
            );
        }
    }

    // Example 11.36 in "Handbook of Elliptic and HyperElliptic Curve Cryptography"
    // u(X) = X^5 + X^4 + X^2 + X, v(X) = X^10 + X^9 + X^7 + X^6 + X^5 + X^4 + X^3 + 1
    // w(X) = u(X) * v(X) = X^15 + X^13 + X^10 + X^9 + X^7 + X^5 + X^2 + X
    #[test]
    fn test_bigint_mul() {
        let test_data = vec![(
            String::from_str("0b110110").unwrap(),
            String::from_str("0b11011111001").unwrap(),
            (
                String::from_str("0b1010011010100110").unwrap(),
                String::from_str("0b").unwrap(),
            ),
        )];
        for (u_bit_string, v_bit_string, (w_low_bit_string, w_high_bit_string)) in test_data {
            let (u, v, w_expected) = (
                BinaryPolynomial::<4>::from_bit_string(&u_bit_string),
                BinaryPolynomial::<4>::from_bit_string(&v_bit_string),
                BinaryPolynomial2([
                    BinaryPolynomial::<4>::from_bit_string(&w_low_bit_string),
                    BinaryPolynomial::<4>::from_bit_string(&w_high_bit_string),
                ]),
            );
            let w = u * v;
            assert_eq!(w, w_expected, "Test for BinaryPolynomial::mul failed!");
            assert!(
                (w.0[0].to_bit_string() == w_low_bit_string)
                    && (w.0[1].to_bit_string() == w_high_bit_string)
            );
        }
    }

    #[test]
    fn test_bigint_mul2() {
        let test_data = vec![(
            String::from_str("0x0074").unwrap(),
            String::from_str("0x06f9").unwrap(),
            (
                String::from_str("0x1514").unwrap(),
                String::from_str("0x0001").unwrap(),
            ),
        )];
        for (u_hex_string, v_hex_string, (w_low_hex_string, w_high_hex_string)) in test_data {
            let (u, v, w_expected) = (
                BinaryPolynomial::<2>::from_hex_string(&u_hex_string),
                BinaryPolynomial::<2>::from_hex_string(&v_hex_string),
                BinaryPolynomial2([
                    BinaryPolynomial::<2>::from_hex_string(&w_low_hex_string),
                    BinaryPolynomial::<2>::from_hex_string(&w_high_hex_string),
                ]),
            );
            let w = u * v;
            assert_eq!(w, w_expected);
            assert_eq!(w.0[0].to_hex_string(), w_low_hex_string);
            assert_eq!(w.0[1].to_hex_string(), w_high_hex_string);
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
                String::from_str("0b11011111001").unwrap(),
                [
                    String::from_str("0b0101010101000001").unwrap(),
                    String::from_str("0b0000000000010100").unwrap(),
                ],
            ),
            (
                String::from_str("0b1010010011111001").unwrap(),
                [
                    String::from_str("0b0101010101000001").unwrap(),
                    String::from_str("0b0100010000010000").unwrap(),
                ],
            ),
        ];
        for (v_bit_string, v_squaring_bit_string) in test_data {
            let (v, v_squaring_expected) = (
                BinaryPolynomial::<2>::from_bit_string(&v_bit_string),
                BinaryPolynomial2([
                    BinaryPolynomial::<2>::from_bit_string(&v_squaring_bit_string[0]),
                    BinaryPolynomial::<2>::from_bit_string(&v_squaring_bit_string[1]),
                ]),
            );
            let v_squaring = v.squaring();
            assert_eq!(
                v_squaring, v_squaring_expected,
                "Test for BinaryPolynomial::squaring failed!"
            );
        }
    }

    #[test]
    fn test_trunk_add() {
        let u = BinaryPolynomial([249, 6, 0, 0]);
        let v = BinaryPolynomial([32, 5, 0, 0]);
        let w_expected = BinaryPolynomial([249, 38, 5, 0]);
        let w = u.trunc_add(1, v);
        assert_eq!(w, w_expected);
    }
}
