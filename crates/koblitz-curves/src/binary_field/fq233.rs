#![allow(non_snake_case)]

/// Base binary field for K-233 curve
use super::{BinaryField, M, N};
use crate::binary_field::polynomial::{BinaryPolynomial, BinaryPolynomial2, WORD_SIZE};
use core::ops::{Add, Div, Mul, Neg, Shl, Shr, Sub};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Fq233(pub BinaryPolynomial<N>);

#[allow(dead_code, non_snake_case)]
impl Fq233 {
    // convert to hex string
    pub fn to_hex_string(&self) -> String {
        self.0.to_hex_string()
    }

    // initiate from hex string
    pub fn from_hex_string(s: &String) -> Self {
        assert!(s.starts_with("0x"));
        let hex_string = s.strip_prefix("0x").unwrap().to_string();
        let mut result = BinaryPolynomial2::<N>::zero();
        let hex_number_boundary = N * WORD_SIZE / 4;
        if hex_string.len() > hex_number_boundary {
            result.0[0] = BinaryPolynomial::<N>::from_hex_string(&format!(
                "0x{}",
                hex_string[(hex_string.len() - hex_number_boundary - 1)..].to_string()
            ));
            result.0[1] = BinaryPolynomial::<N>::from_hex_string(&format!(
                "0x{}",
                hex_string[0..(hex_string.len() - hex_number_boundary)].to_string()
            ));
        } else {
            result.0[0] = BinaryPolynomial::<N>::from_hex_string(s);
        }
        Self::reduce(result)
    }

    // get bit value of binary field
    pub fn get(&self, offset: usize) -> u8 {
        assert!(offset < M, "offset is too big!");
        self.0.get(offset)
    }

    // set bit value of binary field
    pub fn set(&mut self, offset: usize, bit: u8) {
        assert!(offset < M, "offset is too big!");
        self.0.set(offset, bit);
    }

    // swap two binary field
    pub fn swap(&mut self, other: &mut Self) {
        let tmp = *self;
        *self = *other;
        *other = tmp;
    }

    // Modular composition of Brent and Kung, Algorithm 11.50 in "Handbook of Elliptic and HyperElliptic Curve Cryptography"
    // compute f(X)^{2^r} (mod m(X)) = f(g(X)) (mod m(X)), where g(X) = X^{2^r}, deg(f) < M, deg(g) < M and deg(m) = M
    fn modular_composition(&self, g: Self) -> Self {
        let k = (M as f32).sqrt().ceil() as usize;
        assert!(k * k <= N * WORD_SIZE, "Modular parameter N is too small!");
        // precompute G_i[X] = 1, g, g^2, g^3, ...,g^{k - 1}
        let mut G = vec![Fq233::zero(); k];
        G[0] = Fq233::one();
        for i in 1..k {
            G[i] = g * G[i - 1];
        }
        let Gk = g * G[k - 1];
        // precompute P_i[X] = 1, g^k, g^{2k}, g^{3k}, ..., g^{(k - 1)k}
        let mut P = vec![Fq233::zero(); k];
        P[0] = Fq233::one();
        for i in 1..k {
            P[i] = Gk * P[i - 1];
        }
        // compute F_i(X) = \sum_{j = 0}^{k - 1} f_{i * k + j} G_j[X]
        let mut F = vec![Fq233::zero(); k];
        for i in 0..k {
            for j in 0..k {
                if i * k + j >= M {
                    continue;
                }
                if self.get(i * k + j) == 1u8 {
                    F[i] = F[i] + G[j];
                }
            }
        }
        // compute R = \sum_{i = 0}^{k - 1} F_i[X] * P_i[X] (mod m(X))
        let mut R = Fq233::zero();
        for i in 0..k {
            R = R + F[i] * P[i];
        }
        R
    }

    // Shoup exponentiation algorithm for binary field, algorithm 11.53 in "Handbook of Elliptic and HyperElliptic Curve Cryptography"
    // compute f(X)^{n(X)} (mod m(X)) = f(X)^{n_0(X) + n_1(X) * t(X) + n_2(X) * t(X)^2 + ... + n_{l - 1}(X) * t(X)^{l - 1}}
    pub fn exp(&self, e: BinaryPolynomial<N>) -> Self {
        assert!(e.degree() < M, "Input parameter n is too big!");
        let r = (M as f64 / (M as f64).log2()).ceil() as usize;
        let n = e.chunks(r);
        let l = n.len();
        // precompute f^{ni}(X) (mod m(X))
        let mut f_pow_2 = vec![Fq233::zero(); r];
        f_pow_2[0] = *self;
        for i in 1..r {
            f_pow_2[i] = f_pow_2[i - 1].squaring();
        }
        let mut f_n = vec![Fq233::one(); l];
        for i in 0..n.len() {
            for j in 0..r {
                let mask = 1 << j;
                if n[i] & mask == mask {
                    f_n[i] = f_n[i] * f_pow_2[j];
                }
            }
        }
        // compute g(X) = X^{2^r} (mod m(X))
        let mut g = Fq233::one() << 1;
        for _ in 1..(r + 1) {
            g = g.squaring();
        }
        // f^n = f^{n_0 + t * n_1 + t^2 * n_2 + ... + t^{l - 1} * n_{l - 1}}
        let mut y = Fq233::one();
        for i in (0..l).rev() {
            y = y.modular_composition(g);
            y = y * f_n[i];
        }
        y
    }

    // Algorithm 2.48 in "Guide to Elliptic Curve Cryptography"
    // Euclidean based binary field inversion
    pub fn inv(&self) -> Self {
        assert!(*self != Self::zero(), "Zero can not be inversed!");
        if self.is_one() {
            return Self::one();
        }
        let (mut u, mut v) = (
            BinaryPolynomial2::<N>::from(self.0),
            BinaryPolynomial2::<N>::from(Self::F),
        );
        let (mut g1, mut g2) = (Fq233::one(), Fq233::zero());
        while u.is_one() == false {
            let mut j = u.degree() as i32 - v.degree() as i32;
            if j < 0 {
                u.swap(&mut v);
                g1.swap(&mut g2);
                j = -j;
            }
            u = u + (v << j as usize);
            g1 = g1 + (g2 << j as usize);
        }
        g1
    }

    pub fn degree(&self) -> usize {
        assert!(self.0.degree() < M - 1, "Degree Invalid");
        self.0.degree()
    }
}

impl Div for Fq233 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        assert!(rhs != Self::zero(), "Denominator should not be zero!");
        self * rhs.inv()
    }
}

impl Add for Fq233 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if self == Self::zero() {
            return rhs;
        }
        if rhs == Self::zero() {
            return self;
        }
        Self(self.0 + rhs.0)
    }
}

impl Shl<usize> for Fq233 {
    type Output = Self;

    fn shl(self, shift: usize) -> Self::Output {
        Self::reduce(BinaryPolynomial2::<N>::from(self.0) << shift)
    }
}

impl Shr<usize> for Fq233 {
    type Output = Self;

    fn shr(self, shift: usize) -> Self::Output {
        Self::reduce(BinaryPolynomial2::<N>::from(self.0) >> shift)
    }
}

impl Mul for Fq233 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        if (self == Self::zero()) || (rhs == Self::zero()) {
            return Self::zero();
        }
        if self == Self::one() {
            return rhs;
        }
        if rhs == Self::one() {
            return self;
        }
        if self == rhs {
            self.squaring()
        } else {
            Self::reduce(self.0 * rhs.0)
        }
    }
}

impl Neg for Fq233 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl Sub<Self> for Fq233 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        if self == Self::zero() {
            return -rhs;
        }
        if rhs == Self::zero() {
            return self;
        }
        Self(self.0 - rhs.0)
    }
}

impl BinaryField<N> for Fq233 {
    // f(X) = X^233 + r(X), where r(X) = X^74 + 1
    const F: BinaryPolynomial<N> = BinaryPolynomial([1, 0, 1024, 0, 0, 0, 0, 512]);
    // \sqrt(X) = X^228 + X^191 + X^154 + X^117 + X^69 + X^32
    const SQ: BinaryPolynomial<N> =
        BinaryPolynomial([0, 1, 32, 2097152, 67108864, 2147483648, 0, 16]);

    // Algorithm 2.42 in "Guide to Elliptic Curve Cryptography"
    fn reduce(ele: BinaryPolynomial2<N>) -> Self {
        assert!(
            ele.degree() <= 2 * M - 2,
            "Degree of binary polynomial is too big."
        );
        let mut C = ele.clone();
        for i in (N..2 * N).rev() {
            C[i - 8] = C[i - 8] ^ (C[i] << 23);
            C[i - 7] = C[i - 7] ^ (C[i] >> 9);
            C[i - 5] = C[i - 5] ^ (C[i] << 1);
            C[i - 4] = C[i - 4] ^ (C[i] >> 31);
        }
        let T = C[7] >> 9;
        C[0] = C[0] ^ T;
        C[2] = C[2] ^ (T << 10);
        C[3] = C[3] ^ (T >> 22);
        C[7] = C[7] & 0x1ff;

        Self(C.lower())
    }
    // trivial checks
    fn is_zero(&self) -> bool {
        *self == Self::zero()
    }
    fn is_one(&self) -> bool {
        *self == Self::one()
    }
    fn is_power_of_2(&self) -> bool {
        let ones = self.bits(true).iter().fold(0, |acc, v| acc + v);
        (ones == 1u8) || (ones == 0u8)
    }
    // convert to little ending bits and remove leading zeros if necessary
    fn bits(&self, remove: bool) -> Vec<u8> {
        self.0.to_le_bits(remove)
    }
    // square root of binary field
    // \sqrt(f(X)) = f_{even} + \sqrt(X) * f_{odd}, where \sqrt(X) is constant
    fn sqrt(&self) -> Self {
        let pair = self.0.split();
        Self(pair[0]) + Self::reduce(pair[1] * Self::SQ)
    }
    // squaring of binary field
    fn squaring(&self) -> Self {
        if self.is_zero() {
            return Self::zero();
        }
        if self.is_one() {
            return Self::one();
        }
        Self::reduce(self.0.squaring())
    }
    // trace of a binary field
    // Tr(x) = x + x^2 + x^{2^2} + x^{2^3} + ... + x^{2^{M - 1}}
    fn trace(&self) -> Self {
        let mut result = *self;
        let mut sq = *self;
        for _ in 1..M {
            sq = sq.squaring();
            result = result + sq;
        }
        result
    }

    fn zero() -> Self {
        Self(BinaryPolynomial::<N>::zero())
    }

    fn one() -> Self {
        Self(BinaryPolynomial::<N>::one())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_fq_reduce() {
        let test_data = [(
            String::from_str("0x000000000000000000000003ba4d15e1e974d9279e5a5c527a157742b845827b")
                .unwrap(),
            String::from_str("0x000000000000000000000003ba4d15e1e974d9279e5a5c527a157742b845827b")
                .unwrap(),
        )];
        for (v_hex_string, v_reduced_hex_string) in test_data {
            let v = Fq233::from_hex_string(&v_hex_string);
            assert_eq!(v.to_hex_string(), v_reduced_hex_string);
        }
    }

    #[test]
    fn test_fq_mul() {
        let test_data = [(
            String::from_str("0x0000003bd4f59063516f81a1621a4d4885e77e0f4693f893b656abe82c4e5c2f")
                .unwrap(),
            String::from_str("0x00000131fb97cdb584763a0dbfe94f6a78ec31d680ecf7c0df07dafb5b418b09")
                .unwrap(),
            String::from_str("0x000000296bc0bc0ead4ade9dfca37c3b5e5a1c622511d6b765347d7c2de7103d")
                .unwrap(),
        )];
        for (u_hex_string, v_hex_string, w_expected_hex_string) in test_data {
            let (u, v, w_expected) = (
                Fq233::from_hex_string(&u_hex_string),
                Fq233::from_hex_string(&v_hex_string),
                Fq233::from_hex_string(&w_expected_hex_string),
            );
            let w = u * v;
            assert_eq!(w, w_expected, "Test for Fq233 multiplication failed!");
            assert_eq!(w.to_hex_string(), w_expected_hex_string);
        }
    }

    #[test]
    fn test_fq_inv() {
        let test_data = [
            (
                String::from_str(
                    "0x0000003bd4f59063516f81a1621a4d4885e77e0f4693f893b656abe82c4e5c2f",
                )
                .unwrap(),
                String::from_str(
                    "0x000001ecfca5ace9b696238406aab3cf75090c2e7a4ae879be9f29bea5e704b6",
                )
                .unwrap(),
            ),
            (
                String::from_str(
                    "0x00000131fb97cdb584763a0dbfe94f6a78ec31d680ecf7c0df07dafb5b418b09",
                )
                .unwrap(),
                String::from_str(
                    "0x0000001a8bf742ce2424dbaf0e9f0cb042100054afe65f14cff0610b2699da90",
                )
                .unwrap(),
            ),
            (
                String::from_str(
                    "0x000000296bc0bc0ead4ade9dfca37c3b5e5a1c622511d6b765347d7c2de7103d",
                )
                .unwrap(),
                String::from_str(
                    "0x00000160aea7fd976ac242795c52166c71349481dd997e89eaa182f9294dc4b6",
                )
                .unwrap(),
            ),
        ];

        for (u_hex_string, u_inv_expected_hex_string) in test_data {
            let (u, u_inv_expected) = (
                Fq233::from_hex_string(&u_hex_string),
                Fq233::from_hex_string(&u_inv_expected_hex_string),
            );
            let u_inv = u.inv();
            assert_eq!(u_inv, u_inv_expected, "Test for Fq233 inversion failed!");
        }
    }

    #[test]
    fn test_modular_composition() {
        let test_data = [
            (
                String::from_str(
                    "0x000000f2b074776e507205cd40b5eb706c989deef9b76912c7e23b9bbad84433",
                )
                .unwrap(),
                6,
                String::from_str(
                    "0x00000030ad418b174faeb0a6007c045c548d6d11eb99dac929cf3d4d100e1755",
                )
                .unwrap(),
            ),
            (
                String::from_str(
                    "0x000000f2b074776e507205cd40b5eb706c989deef9b76912c7e23b9bbad84433",
                )
                .unwrap(),
                4,
                String::from_str(
                    "0x0000010b0bc4030227274e9903596a15192d17d81aa579be95df4e26d9365849",
                )
                .unwrap(),
            ),
        ];
        for (u_hex_string, r, u_exp_hex_string) in test_data {
            let (u, u_exp_expected) = (
                Fq233::from_hex_string(&u_hex_string),
                Fq233::from_hex_string(&u_exp_hex_string),
            );
            assert!((1 << r) < M, "Input parameter r is too big!");
            // g[X] = X^{2^r}
            let g = {
                let mut v = Fq233::one() << 1;
                for _ in 1..(r + 1) {
                    v = v.squaring();
                }
                v
            };
            let u_exp = u.modular_composition(g);
            assert_eq!(
                u_exp, u_exp_expected,
                "Test for Fq233 modular composition failed!"
            );
        }
    }

    #[test]
    fn test_exp() {
        let test_data = [
            (
                String::from_str(
                    "0x000000f2b074776e507205cd40b5eb706c989deef9b76912c7e23b9bbad84433",
                )
                .unwrap(),
                String::from_str(
                    "0x000000dbd55057dd12413fb25a6d4189b1109905a55dca6038eed1ffce235d34",
                )
                .unwrap(),
                String::from_str(
                    "0x000000754e2c4a1912c4fecfdba7184369a36b68e29315b6a9962fa652c9eb8e",
                )
                .unwrap(),
            ),
            (
                String::from_str(
                    "0x000000f2b074776e507205cd40b5eb706c989deef9b76912c7e23b9bbad84433",
                )
                .unwrap(),
                String::from_str(
                    "0x0000012ececad8c345361185c03daba2e541a387e404843f6f9bca5f873a7062",
                )
                .unwrap(),
                String::from_str(
                    "0x00000178d21000676c8880a65f727dd70afae1523c402cee849e36eb51a20fa4",
                )
                .unwrap(),
            ),
        ];
        for (u_hex_string, v_hex_string, u_exp_hex_string) in test_data {
            let (u, v, u_exp_expected) = (
                Fq233::from_hex_string(&u_hex_string),
                BinaryPolynomial::<N>::from_hex_string(&v_hex_string),
                Fq233::from_hex_string(&u_exp_hex_string),
            );
            let u_exp = u.exp(v);
            assert_eq!(u_exp, u_exp_expected, "Test for Fq233 exp failed!");
        }
    }

    #[test]
    fn test_sqrt() {
        let test_data = [
            (
                String::from_str(
                    "0x000000f2b074776e507205cd40b5eb706c989deef9b76912c7e23b9bbad84433",
                )
                .unwrap(),
                String::from_str(
                    "0x0000016b95f7e3f698f8ba15b833af7f40ac4efacc3b854b8f951062010a329a",
                )
                .unwrap(),
            ),
            (
                String::from_str(
                    "0x000000dbd55057dd12413fb25a6d4189b1109905a55dca6038eed1ffce235d34",
                )
                .unwrap(),
                String::from_str(
                    "0x000000b21f87563e10e62accc68df1f6a48dfcff4974caf56b5d93e36b27d495",
                )
                .unwrap(),
            ),
        ];
        assert_eq!(
            Fq233::reduce(Fq233::SQ.squaring()),
            Fq233::one() << 1,
            "Square root of X is not correct!"
        );
        for (u_hex_string, u_sqrt_hex_string) in test_data {
            let (u, u_sqrt_expected) = (
                Fq233::from_hex_string(&u_hex_string),
                Fq233::from_hex_string(&u_sqrt_hex_string),
            );
            let u_sqrt = u.sqrt();
            assert_eq!(u_sqrt, u_sqrt_expected, "Test for Fq233 sqrt failed!");
        }
    }

    #[test]
    fn test_trace() {
        let test_data = [String::from_str(
            "0x0000013e1039b7c2ad6a0d92c83537b5704dfee0d8ac4243f3aa4e2a79bb7787",
        )
        .unwrap()];
        for u_hex_string in test_data {
            let u = Fq233::from_hex_string(&u_hex_string);
            let w = u.trace();
            assert_eq!(w, Fq233::zero(), "Test for trace of binary field failed!");
        }
    }
}
