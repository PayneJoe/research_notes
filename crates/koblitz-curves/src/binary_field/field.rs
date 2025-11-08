use crate::binary_field::polynomial::{
    BinaryField, BinaryPolynomial, BinaryPolynomial2, WORD_SIZE,
};
use core::ops::{Add, Div, Mul, Neg, Shl, Shr, Sub};

// binary field Fq = GF(2^m) / f(X), where m = 163
// N = 24 when word = u8
pub const M: usize = 163;
pub const N: usize = 24;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Fq(BinaryPolynomial<N>);

#[allow(dead_code)]
impl Fq {
    pub fn to_hex_string(&self) -> String {
        self.0.to_hex_string()
    }

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

    pub fn zero() -> Self {
        Self(BinaryPolynomial::<N>::zero())
    }

    pub fn one() -> Self {
        Self(BinaryPolynomial::<N>::one())
    }

    pub fn squaring(&self) -> Self {
        Self::reduce(self.0.squaring())
    }

    pub fn swap(&mut self, other: &mut Self) {
        let tmp = *self;
        *self = *other;
        *other = tmp;
    }

    // Algorithm 2.48 in "Guide to Elliptic Curve Cryptography"
    // Euclidean based binary field inversion
    pub fn inv(&self) -> Self {
        assert!(*self != Self::zero(), "Zero can not be inversed!");
        let (mut u, mut v) = (
            BinaryPolynomial2::<N>::from(self.0),
            BinaryPolynomial2::<N>::from(Self::F),
        );
        let (mut g1, mut g2) = (Fq::one(), Fq::zero());
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

    // // Algorithm 11.44 in "Handbook of Elliptic and HyperElliptic Curve Crptography"
    // pub fn inv(&self) -> Self {
    //     assert!(*self != Self::zero(), "Zero can not be inversed!");
    //     let d = Self::M;
    //     let mut f = BinaryPolynomial2::<N>::from(self.0);
    //     let (mut u, mut v, mut s, mut delta) = (
    //         Self::one(),
    //         Self::zero(),
    //         BinaryPolynomial2::<N>::from(Self::F),
    //         0 as usize,
    //     );
    //     for _ in 1..(2 * d + 1) {
    //         if f.bit(d) == 0 {
    //             f = f << 1;
    //             u = u << 1;
    //             delta += 1;
    //         } else {
    //             if s.bit(d) == 1 {
    //                 s = s - f;
    //                 v = v - u;
    //             }
    //             s = s << 1;
    //             if delta == 0 {
    //                 // f <-> s
    //                 f.swap(&mut s);
    //                 // u <-> v
    //                 u.swap(&mut v);
    //                 u = u << 1;
    //                 delta = 1;
    //             } else {
    //                 u = u >> 1;
    //                 delta -= 1;
    //             }
    //         }
    //     }
    //     u
    // }

    pub fn degree(&self) -> usize {
        assert!(self.0.degree() < M - 1, "Degree Invalid");
        self.0.degree()
    }
}

impl Div for Fq {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        assert!(rhs != Self::zero(), "Denominator should not be zero!");
        self * rhs.inv()
    }
}

impl Add for Fq {
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

impl Shl<usize> for Fq {
    type Output = Self;

    fn shl(self, shift: usize) -> Self::Output {
        Self::reduce(BinaryPolynomial2::<N>::from(self.0) << shift)
    }
}

impl Shr<usize> for Fq {
    type Output = Self;

    fn shr(self, shift: usize) -> Self::Output {
        Self::reduce(BinaryPolynomial2::<N>::from(self.0) >> shift)
    }
}

impl Mul for Fq {
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
        if self == Self::zero() {
            return -rhs;
        }
        if rhs == Self::zero() {
            return self;
        }
        Self(self.0 - rhs.0)
    }
}

impl BinaryField<N> for Fq {
    const M: usize = M;
    const F: BinaryPolynomial<N> = BinaryPolynomial([
        201, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0,
    ]);
    const R: BinaryPolynomial<N> = BinaryPolynomial([
        201, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ]);
    const UK: [BinaryPolynomial<N>; WORD_SIZE] = [
        BinaryPolynomial([
            201, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ]),
        BinaryPolynomial([
            146, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ]),
        BinaryPolynomial([
            36, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ]),
        BinaryPolynomial([
            72, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ]),
        BinaryPolynomial([
            144, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ]),
        BinaryPolynomial([
            32, 25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ]),
        BinaryPolynomial([
            64, 50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ]),
        BinaryPolynomial([
            128, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ]),
    ];

    // Algorithm 2.40 in "Guide to Elliptic Curve Cryptography"
    fn reduce(ele: BinaryPolynomial2<N>) -> Self {
        let pair = ele.split_at(M);
        let mut c = pair[0];

        // deal with overflow bit-by-bit
        for i in (0..M - 1).rev() {
            let j = i / WORD_SIZE;
            let k = i - j * WORD_SIZE;
            if pair[1].get_bit(j, k) == 1u8 {
                c = c.trunc_add(j, Self::UK[k]);
            }
        }

        // deal with one more word overflow
        for i in M..(N * WORD_SIZE) {
            let j = i / WORD_SIZE;
            let k = i - j * WORD_SIZE;
            if c.get_bit(j, k) == 1u8 {
                // you should be here, if your constant parameters is choosing approiately
                if i + 1 - M >= WORD_SIZE {
                    assert!(false, "Overflow Error: the degree of r(x) is too big!");
                }
                c = Self::UK[i - M] + c;
                // clear overflow bits
                c.set_bit(j, k, 0u8);
            }
        }

        Self(c)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_fq_reduce() {
        let test_data = [(
            String::from_str("0x00000003ba4d15e1e974d9279e5a5c527a157742b845827b").unwrap(),
            String::from_str("0x00000003ba4d15e1e974d9279e5a5c527a157742b845827b").unwrap(),
        )];
        for (v_hex_string, v_reduced_hex_string) in test_data {
            let v = Fq::from_hex_string(&v_hex_string);
            assert_eq!(v.to_hex_string(), v_reduced_hex_string);
        }
    }

    #[test]
    fn test_fq_mul() {
        for i in 0..Fq::UK.len() - 1 {
            assert_eq!(Fq::UK[i] << 1, Fq::UK[i + 1]);
        }
        let test_data = [
            (
                String::from_str("0x00000003ba4d15e1e974d9279e5a5c527a157742b845827b").unwrap(),
                String::from_str("0x000000001d4350a888ab13aacc54664d0f1f7ebb315f8039").unwrap(),
                String::from_str("0x000000033b74e65b81aeadbe2bfc6968ed3c050d10363e8c").unwrap(),
            ),
            (
                String::from_str("0x0000000644192702d2623c11c05c3196ee6490c8f4927ce5").unwrap(),
                String::from_str("0x00000004ef895f49b9b91e352a6c05dd3136d6e5249dae50").unwrap(),
                String::from_str("0x00000006b15a564aaf5e7df8d4424c03bc35bd7c2c61e17e").unwrap(),
            ),
        ];
        for (u_hex_string, v_hex_string, w_expected_hex_string) in test_data {
            let (u, v, w_expected) = (
                Fq::from_hex_string(&u_hex_string),
                Fq::from_hex_string(&v_hex_string),
                Fq::from_hex_string(&w_expected_hex_string),
            );
            let w = u * v;
            assert_eq!(w, w_expected, "Test for Fq multiplication failed!");
            assert_eq!(w.to_hex_string(), w_expected_hex_string);
        }
    }

    #[test]
    fn test_fq_inv() {
        let test_data = [
            (
                String::from_str("0x0000000644192702d2623c11c05c3196ee6490c8f4927ce5").unwrap(),
                String::from_str("0x00000002fbca818f7ebbfb003b6318ff7c959ed1281782ea").unwrap(),
            ),
            (
                String::from_str("0x00000004ef895f49b9b91e352a6c05dd3136d6e5249dae50").unwrap(),
                String::from_str("0x000000031e49c95bfa16c6b85d5cd2c3abe1e2f071c68e92").unwrap(),
            ),
            (
                String::from_str("0x00000006b15a564aaf5e7df8d4424c03bc35bd7c2c61e17e").unwrap(),
                String::from_str("0x00000000c9b5d404e46e8e8db628198f32a1f3f8972dab36").unwrap(),
            ),
        ];

        for (u_hex_string, u_inv_expected_hex_string) in test_data {
            let (u, u_inv_expected) = (
                Fq::from_hex_string(&u_hex_string),
                Fq::from_hex_string(&u_inv_expected_hex_string),
            );
            let u_inv = u.inv();
            assert_eq!(u_inv, u_inv_expected, "Test for Fq inversion failed!");
        }
    }
}
