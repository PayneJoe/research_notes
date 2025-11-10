#![allow(dead_code)]
use super::BinaryWord;
use super::word::*;
use hex;
use std::fmt::Debug;
use std::ops::{Add, Mul, Neg, Shl, Shr, Sub};

// u8 word only for testing purpose, actually we will use u32 or u64 as one word
pub type WORD = WORD32;
pub const WORD_SIZE: usize = 32;
// window size for caching when doing bigint multiplication
pub const WINDOW_SIZE: usize = 4;

// binary polynomial representation for bigint
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct BinaryPolynomial<const N: usize>(pub [WORD; N]);

#[allow(dead_code)]
impl<const N: usize> BinaryPolynomial<N> {
    // split BinaryPolynimal2 into a even and odd BinaryPolynomial,
    // this is for the purpose of sqrt
    pub fn split(&self) -> [BinaryPolynomial<N>; 2] {
        let bit_vec = self.to_be_bits();
        let even = bit_vec.iter().step_by(2).map(|b| *b).collect::<Vec<_>>();
        let odd = bit_vec
            .iter()
            .skip(1)
            .step_by(2)
            .map(|b| *b)
            .collect::<Vec<_>>();
        [
            BinaryPolynomial::from_be_bits(even),
            BinaryPolynomial::from_be_bits(odd),
        ]
    }

    // to bit vec with big ending order
    pub fn to_be_bits(&self) -> Vec<u8> {
        self.0
            .map(|w| w.to_le_bytes().map(|b| b.to_be_bits()).concat())
            .concat()
    }

    // chunk a binary polynomial with a specific size
    pub fn chunks(&self, size: usize) -> Vec<u32> {
        assert!(size <= 32, "chunk size is too big!");
        let n = ((N * WORD_SIZE) as f32 / size as f32).ceil() as usize;
        let mut result = vec![0u32; n];
        let (mut word, mut word_mask) = (0u32, 1u32);
        for i in 0..(N * WORD_SIZE) {
            if (i > 0) && (i % size == 0) {
                result[i / size - 1] = word;
                (word, word_mask) = (0u32, 1u32);
            }
            if self.get(i) == 1u8 {
                word += word_mask;
            }
            word_mask <<= 1;
        }
        result[n - 1] = word;
        result
    }

    // set one bit in binary polynomial
    pub fn set_bit(&mut self, word_offset: usize, bit_offset: usize, bit: u8) {
        assert!((word_offset < N) && (bit_offset < WORD_SIZE));
        assert!((bit == 0u8) || (bit == 1u8));
        let word_mask = 1 << bit_offset;
        if bit == 1u8 {
            self.0[word_offset] |= word_mask;
        } else {
            self.0[word_offset] &= !word_mask;
        }
    }

    // get one bit in binary polynomial
    pub fn get_bit(&self, word_offset: usize, bit_offset: usize) -> u8 {
        assert!((word_offset < N) && (bit_offset < WORD_SIZE));
        let word_mask = 1 << bit_offset;
        if (self.0[word_offset] & word_mask) == word_mask {
            1u8
        } else {
            0u8
        }
    }

    // a simpler method for get_bit
    pub fn get(&self, offset: usize) -> u8 {
        assert!(offset < N * WORD_SIZE);
        let (word_offset, bit_offset) = (offset / WORD_SIZE, offset % WORD_SIZE);
        self.get_bit(word_offset, bit_offset)
    }

    // a simpler method for set_bit
    pub fn set(&mut self, offset: usize, bit: u8) {
        assert!(offset < N * WORD_SIZE);
        let (word_offset, bit_offset) = (offset / WORD_SIZE, offset % WORD_SIZE);
        self.set_bit(word_offset, bit_offset, bit);
    }

    // to hex bytes string
    pub fn to_hex_string(&self) -> String {
        let bytes = self
            .0
            .iter()
            .map(|w| hex::encode(w.to_be_bytes()))
            .rev()
            .collect::<Vec<_>>()
            .concat();
        format!("0x{}", bytes)
    }

    // from little ending hex bytes string
    pub fn from_hex_string(s: &String) -> Self {
        assert!(s.starts_with("0x"));

        let bytes = hex::decode(s.chars().skip(2).collect::<String>())
            .expect("Invalid hex string")
            .into_iter()
            .collect::<Vec<_>>();

        let words = bytes
            .chunks(WORD_SIZE / 8)
            .map(|v| WORD::from_be_bytes(v.try_into().unwrap()))
            .rev()
            .collect::<Vec<_>>();
        Self::from(words)
    }

    // to little ending bit string
    pub fn to_bit_string(&self) -> String {
        let bit_string = self
            .to_be_bits()
            .iter()
            .map(|b| if *b == 1u8 { '1' } else { '0' })
            .rev()
            .collect::<String>();
        format!("0b{}", bit_string.trim_start_matches("0"))
    }

    // from bit vec with big ending
    pub fn from_be_bits(bit_vec: Vec<u8>) -> Self {
        let n = bit_vec.len();
        assert!(n <= N * WORD_SIZE);
        if n == 0 {
            return Self::zero();
        }
        let mut result = Self::zero();
        let (mut w, mut w_mask) = (0 as WORD, 1 as WORD);
        for i in 0..n {
            if (i > 0) && (i % WORD_SIZE == 0) {
                result.0[i / WORD_SIZE - 1] = w;
                (w, w_mask) = (0 as WORD, 1 as WORD);
            }
            if bit_vec[i] == 1u8 {
                w += w_mask;
            }
            w_mask <<= 1;
        }
        result.0[(n - 1) / WORD_SIZE] = w;
        result
    }

    // from bit string with little ending
    pub fn from_bit_string(s: &String) -> Self {
        assert!(s.starts_with("0b"));
        let bit_vec = s
            .chars()
            .skip(2)
            .into_iter()
            .map(|c| if c == '1' { 1u8 } else { 0u8 })
            .collect::<Vec<_>>();
        Self::from_be_bits(bit_vec.into_iter().rev().collect::<Vec<_>>())
    }

    // the degree of binary polynomial
    pub fn degree(&self) -> usize {
        if self.is_zero() {
            return 0;
        }
        let mut n = 0 as usize;
        for i in (0..N).rev() {
            let zeros = self.0[i].leading_zeros() as usize;
            if (n % WORD_SIZE == 0) && (zeros < WORD_SIZE) {
                n += zeros;
                break;
            }
            n += WORD_SIZE;
        }
        N * WORD_SIZE - 1 - n
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
        *self == Self::zero()
    }

    pub fn is_one(&self) -> bool {
        *self == Self::one()
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
            let word_bytes = self.0[i].to_le_bytes();
            for byte in word_bytes {
                let byte_squaring = lookup_table[byte as usize];
                result.push(byte_squaring[0]);
                result.push(byte_squaring[1]);
            }
        }
        let words = result
            .chunks(WORD_SIZE / 8)
            .map(|v| WORD::from_le_bytes(v.try_into().unwrap()))
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
        let num_words = N;
        let mut result = vec![0 as WORD; num_words];
        let word_shift = shift / WORD_SIZE;
        let bit_shift = shift % WORD_SIZE;

        for i in (0..num_words).rev() {
            if i >= word_shift {
                result[i] = self.0[i - word_shift] << bit_shift;
                if bit_shift > 0 && i - word_shift > 0 {
                    result[i] |= self.0[i - word_shift - 1] >> (WORD_SIZE - bit_shift);
                }
            }
        }
        BinaryPolynomial::from(result)
    }
}

impl<const N: usize> Shr<usize> for BinaryPolynomial<N> {
    type Output = Self;

    fn shr(self, shift: usize) -> Self::Output {
        let num_words = N;
        let mut result = vec![0 as WORD; num_words];
        let word_shift = shift / WORD_SIZE;
        let bit_shift = shift % WORD_SIZE;

        for i in 0..num_words {
            if i >= word_shift {
                result[num_words - 1 - i] = self.0[num_words - 1 - i + word_shift] >> bit_shift;
                if bit_shift > 0 && i - word_shift > 0 {
                    result[num_words - 1 - i] |=
                        self.0[num_words - i + word_shift] << (WORD_SIZE - bit_shift);
                }
            }
        }
        BinaryPolynomial::from(result)
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
    pub fn is_one(&self) -> bool {
        self.0[1].is_zero() && self.0[0].is_one()
    }

    pub fn is_zero(&self) -> bool {
        self.0[0].is_zero() && self.0[1].is_zero()
    }

    // get the degree of binary polynomial
    pub fn degree(&self) -> usize {
        let high_degree = self.0[1].degree();
        if high_degree > 0 {
            high_degree + N
        } else {
            self.0[0].degree()
        }
    }

    // get one bit of specific offset
    pub fn get(&self, offset: usize) -> u8 {
        assert!(offset < 2 * N * WORD_SIZE);
        if offset < N {
            self.0[0].get(offset)
        } else {
            self.0[1].get(offset - N)
        }
    }

    // swap two binary polynomials
    pub fn swap(&mut self, other: &mut Self) {
        let tmp = *self;
        *self = *other;
        *other = tmp;
    }

    // split a BinaryPolynomial2 at a index within the lower one,
    // this is for the purpose of reducing a BinaryPolynomial2 to a BinaryPolynomial
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

impl<const N: usize> From<Vec<WORD>> for BinaryPolynomial2<N> {
    fn from(v: Vec<WORD>) -> Self {
        if v.len() <= N {
            BinaryPolynomial2::<N>::from(BinaryPolynomial::<N>::from(v))
        } else {
            BinaryPolynomial2([
                BinaryPolynomial::<N>::from(v[..N].to_vec()),
                BinaryPolynomial::<N>::from(v[N..].to_vec()),
            ])
        }
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

impl<const N: usize> Sub<Self> for BinaryPolynomial2<N> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self([self.0[0] - rhs.0[0], self.0[1] - rhs.0[1]])
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
        let num_words = 2 * N;
        let mut result = vec![0 as WORD; num_words];
        let word_shift = shift / WORD_SIZE;
        let bit_shift = shift % WORD_SIZE;

        let mut words = self.0[0].0.to_vec();
        words.extend_from_slice(&self.0[1].0);

        for i in (0..num_words).rev() {
            if i >= word_shift {
                result[i] = words[i - word_shift] << bit_shift;
                if bit_shift > 0 && i - word_shift > 0 {
                    result[i] |= words[i - word_shift - 1] >> (WORD_SIZE - bit_shift);
                }
            }
        }
        Self::from(result)
    }
}

impl<const N: usize> Shr<usize> for BinaryPolynomial2<N> {
    type Output = Self;

    fn shr(self, shift: usize) -> Self::Output {
        let num_words = 2 * N;
        let mut result = vec![0 as WORD; num_words];
        let word_shift = shift / WORD_SIZE;
        let bit_shift = shift % WORD_SIZE;

        let mut words = self.0[0].0.to_vec();
        words.extend_from_slice(&self.0[1].0);

        for i in 0..num_words {
            if i >= word_shift {
                result[num_words - 1 - i] = words[num_words - 1 - i + word_shift] >> bit_shift;
                if bit_shift > 0 && i - word_shift > 0 {
                    result[num_words - 1 - i] |=
                        words[num_words - i + word_shift] << (WORD_SIZE - bit_shift);
                }
            }
        }

        Self::from(result)
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
    const F: BinaryPolynomial<N>;
    // degree(R(X)) = k which is a small odd number
    const R: BinaryPolynomial<N>;
    // uk = R(X), R(X) << 1, R(X) << 2, ..., R(X) << WORD_SIZE - 1
    const UK: [BinaryPolynomial<N>; WORD_SIZE];
    // sqrt(X) = X^{(M + 1) / 2} + X^((k + 1) / 2) when irreducible polynomial m(X) is a trinomial X^M + x^k + 1 and k is a odd number
    // Note that it works for K-233 field, not works for K-163 field
    const SQ: BinaryPolynomial<N>;
    fn reduce(element: BinaryPolynomial2<N>) -> Self;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;
    const N: usize = 6;

    #[test]
    fn test_hex_string() {
        let test_data = [(
            String::from_str("0x0000000644192702d2623c11c05c3196ee6490c8f4927ce5").unwrap(),
            BinaryPolynomial([
                4103240933, 3999568072, 3227267478, 3529653265, 1142499074, 6,
            ]),
        )];
        for (v_hex_string, v_expected) in test_data {
            let v = BinaryPolynomial::<N>::from_hex_string(&v_hex_string);
            assert_eq!(v, v_expected);
            assert_eq!(v.to_hex_string(), v_hex_string);
        }
    }

    // test bit string with big ending (high bit in the end) conversion from/to bigint
    #[test]
    fn test_bit_string() {
        let test_data = [(
            String::from_str("0b1100100010000011001001001110000001011010010011000100011110000010001110000000101110000110001100101101110111001100100100100001100100011110100100100100111110011100101")
                .unwrap(),
            BinaryPolynomial([
                4103240933, 3999568072, 3227267478, 3529653265, 1142499074, 6,
            ]),
        )];
        for (v_bit_string, v_expected) in test_data {
            let v = BinaryPolynomial::<N>::from_bit_string(&v_bit_string);
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

    #[test]
    fn test_binary_polynomial_mul() {
        let test_data = vec![(
            String::from_str("0xee6490c8f4927ce5").unwrap(),
            String::from_str("0xd2623c11c05c3196").unwrap(),
            (
                String::from_str("0xbbbd17f4e2f6a38e").unwrap(),
                String::from_str("0x43b672ea7a185e50").unwrap(),
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
    fn test_binary_polynomial_squaring() {
        let test_data = [
            (
                String::from_str("0xee6490c8f4927ce5").unwrap(),
                [
                    String::from_str("0x5510410415505411").unwrap(),
                    String::from_str("0x5454141041005040").unwrap(),
                ],
            ),
            (
                String::from_str("0xd2623c11c05c3196").unwrap(),
                [
                    String::from_str("0x5000115005014114").unwrap(),
                    String::from_str("0x5104140405500101").unwrap(),
                ],
            ),
        ];
        for (v_hex_string, v_squaring_hex_string) in test_data {
            let (v, v_squaring_expected) = (
                BinaryPolynomial::<2>::from_hex_string(&v_hex_string),
                BinaryPolynomial2([
                    BinaryPolynomial::<2>::from_hex_string(&v_squaring_hex_string[0]),
                    BinaryPolynomial::<2>::from_hex_string(&v_squaring_hex_string[1]),
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
