#![allow(dead_code, non_snake_case)]

pub mod k233;

use crate::binary_field::BinaryField;
use core::ops::{Add, Neg, Sub};
use std::fmt::Debug;
use std::marker::PhantomData;

// General projective coordinates
pub struct Point<const N: usize, Field: BinaryField<N>, Curve: BinaryCurve<N, Field>> {
    x: Field,
    y: Field,
    z: Field,
    marker: PhantomData<Curve>,
}

impl<const N: usize, Field: BinaryField<N>, Curve: BinaryCurve<N, Field>> Point<N, Field, Curve> {
    fn is_affine(&self) -> bool {
        self.z.is_one()
    }
}

impl<const N: usize, Field: BinaryField<N>, Curve: BinaryCurve<N, Field>> Add
    for Point<N, Field, Curve>
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Curve::add(&self, &rhs)
    }
}

impl<const N: usize, Field: BinaryField<N>, Curve: BinaryCurve<N, Field>> Sub
    for Point<N, Field, Curve>
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        self + (-rhs)
    }
}

impl<const N: usize, Field: BinaryField<N>, Curve: BinaryCurve<N, Field>> Neg
    for Point<N, Field, Curve>
{
    type Output = Self;

    fn neg(self) -> Self {
        Curve::neg(&self)
    }
}

// Binary curve with short Weierstrass equation: y^2 + xy = x^3 + a_2 * x^2 + a_6
// In terms of K-233 curve, a_2 = 0, a_6 = 1
pub trait BinaryCurve<const N: usize, Field: BinaryField<N>>:
    Debug + Clone + Copy + Sized + Eq + PartialEq
{
    const A2: Field;
    const A6: Field;
    const IDENTITY: Point<N, Field, Self>;
    const GENERATOR: Point<N, Field, Self>;

    // Lopez-Dahab Coordinates based point addition
    fn add(lft: &Point<N, Field, Self>, rhs: &Point<N, Field, Self>) -> Point<N, Field, Self> {
        let ((X1, Y1, Z1), (X2, Y2, Z2)) = ((lft.x, lft.y, lft.z), (rhs.x, rhs.y, rhs.z));
        // mixed Coordinates
        if rhs.is_affine() {
            let (A, B) = (Y1 + Y2 * (Z1 * Z1), X1 + X2 * Z1);
            let C = B * Z1;
            let Z3 = C * C;
            let D = X2 * Z3;
            let X3 = A * A + C * (A + B * B + Self::A2 * C);
            let Y3 = (D + X3) * (A * C + Z3) + (Y2 + X2) * (Z3 * Z3);
            Point {
                x: X3,
                y: Y3,
                z: Z3,
                marker: PhantomData::<Self>,
            }
        } else {
            let (A, B) = (X1 * Z2, X2 * Z1);
            let (C, D, E) = (A * A, B * B, A + B);
            let F = C + D;
            let (G, H) = (Y1 * (Z2 * Z2), Y2 * (Z1 * Z1));
            let I = G + H;
            let J = I * E;
            let Z3 = F * Z1 * Z2;
            let X3 = A * (H + D) + B * (C + G);
            let Y3 = (A * J + F * G) * F + (J + Z3) * X3;
            Point {
                x: X3,
                y: Y3,
                z: Z3,
                marker: PhantomData::<Self>,
            }
        }
    }

    // two solutions for y when x is fixed, i.e two points which are negative with each other
    fn neg(lft: &Point<N, Field, Self>) -> Point<N, Field, Self> {
        Point {
            x: lft.x,
            y: lft.y + lft.x * lft.z,
            z: lft.z,
            marker: PhantomData::<Self>,
        }
    }
}
