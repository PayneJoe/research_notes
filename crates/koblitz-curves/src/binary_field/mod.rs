#![allow(dead_code)]
pub mod fq233;
pub mod polynomial;
pub mod word;

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
    // degree(R(X)) = k which is a small odd number
    const R: BinaryPolynomial<N>;
    // uk = R(X), R(X) << 1, R(X) << 2, ..., R(X) << WORD_SIZE - 1
    const UK: [BinaryPolynomial<N>; WORD_SIZE];
    // sqrt(X) = X^{(M + 1) / 2} + X^((k + 1) / 2) when irreducible polynomial m(X) is a trinomial X^M + x^k + 1 and k is a odd number
    const SQ: BinaryPolynomial<N>;
    fn reduce(element: BinaryPolynomial2<N>) -> Self;
}

#[allow(dead_code)]
pub trait BinaryWord: Sized {
    fn squaring(&self) -> [Self; 2];
    fn to_be_bits(&self) -> Vec<u8>;
}
