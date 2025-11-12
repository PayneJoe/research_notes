#![allow(dead_code)]
pub mod fq233;
pub mod fr233;
pub mod polynomial;
pub mod word;

// binary field Fq233 = GF(2^m) / f(X), where m = 233 and f(X) = X^233 + X^74 + 1
// N = 8 when word = u32
pub const M: usize = 233;
pub const N: usize = 8;

use polynomial::{BinaryPolynomial, BinaryPolynomial2, WORD_SIZE};
use std::fmt::Debug;
use std::ops::{Add, Div, Mul, Neg, Sub};

pub trait BinaryField<const N: usize>:
    Debug
    + Eq
    + PartialEq
    + Copy
    + Clone
    + Add<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Mul<Self, Output = Self>
    + Div<Self, Output = Self>
    + Neg<Output = Self>
{
    // irreducible binary polynomial: f(X) = X^M + R(X) where M is the degree of binary polynomial, and R(X) is residual polynomial
    // which M <= N * WORD_SIZE, and deg(R) < M
    const M: usize;
    const F: BinaryPolynomial<N>;
    // sqrt(X) = X^{(M + 1) / 2} + X^((k + 1) / 2) when irreducible polynomial m(X) is a trinomial X^M + x^k + 1 and k is a odd number
    const SQ: BinaryPolynomial<N>;
    // reduce a big binary polynomial with a fixed irreducible binary polynomial with degree M
    fn reduce(element: BinaryPolynomial2<N>) -> Self;
    fn one() -> Self;
    fn zero() -> Self;
    fn is_zero(&self) -> bool;
    fn is_one(&self) -> bool;
    fn is_power_of_2(&self) -> bool;
    fn bits(&self, remove: bool) -> Vec<u8>;
    fn sqrt(&self) -> Self;
    fn squaring(&self) -> Self;
    fn trace(&self) -> Self;
}

#[allow(dead_code)]
pub trait BinaryWord: Sized {
    fn squaring(&self) -> [Self; 2];
    fn to_le_bits(&self) -> Vec<u8>;
}
