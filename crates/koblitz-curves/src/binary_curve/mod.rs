#![allow(dead_code, non_snake_case)]

pub mod k233;

use crate::binary_field::BinaryField;
use core::ops::{Add, Mul, Neg, Sub};
use std::fmt::Debug;
use std::marker::PhantomData;

// General projective coordinates which is not relevant with the short Weierstrass equation
#[derive(Debug, Clone, Copy)]
pub struct ProjectivePoint<const N: usize, Field: BinaryField<N>, Curve: BinaryCurve<N, Field>> {
    x: Field,
    y: Field,
    z: Field,
    marker: PhantomData<Curve>,
}

impl<const N: usize, Field: BinaryField<N>, Curve: BinaryCurve<N, Field>> Eq
    for ProjectivePoint<N, Field, Curve>
{
}

impl<const N: usize, Field: BinaryField<N>, Curve: BinaryCurve<N, Field>> PartialEq
    for ProjectivePoint<N, Field, Curve>
{
    fn eq(&self, other: &Self) -> bool {
        if self.is_identity() && other.is_identity() {
            return true;
        }
        if self.is_identity() || other.is_identity() {
            return false;
        }
        ((self.x / self.z) == (other.x / other.z))
            && ((self.y / self.z.squaring()) == (other.y / other.z.squaring()))
    }
}

impl<const N: usize, Field: BinaryField<N>, Curve: BinaryCurve<N, Field>>
    ProjectivePoint<N, Field, Curve>
{
    fn is_affine(&self) -> bool {
        self.z.is_one()
    }

    fn is_identity(&self) -> bool {
        self.z.is_zero()
    }

    fn is_on_curve(&self) -> bool {
        Curve::is_on_curve(self)
    }
}

impl<const N: usize, Field: BinaryField<N>, Curve: BinaryCurve<N, Field>> Mul<Field>
    for ProjectivePoint<N, Field, Curve>
{
    type Output = Self;

    fn mul(self, scalar: Field) -> Self {
        // Curve::montgomery_scalar_mul(&self, scalar)
        Curve::fast_montgomery_scalar_mul(&self, scalar)
    }
}

impl<const N: usize, Field: BinaryField<N>, Curve: BinaryCurve<N, Field>> Add
    for ProjectivePoint<N, Field, Curve>
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        if self == rhs {
            return Curve::double(&self);
        }
        Curve::add(&self, &rhs)
    }
}

impl<const N: usize, Field: BinaryField<N>, Curve: BinaryCurve<N, Field>> Sub
    for ProjectivePoint<N, Field, Curve>
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        self + (-rhs)
    }
}

impl<const N: usize, Field: BinaryField<N>, Curve: BinaryCurve<N, Field>> Neg
    for ProjectivePoint<N, Field, Curve>
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
    const IDENTITY: ProjectivePoint<N, Field, Self>;
    const GENERATOR: ProjectivePoint<N, Field, Self>;
    const A6_SQRT: Field;

    // Regarding Lopez-Dahab projective coordinates, the Weierstrass equation is: Y^2 + X * Y * Z = X^3 * Z + a_2 * X^2 * Z^2 + a_6 * Z^4
    fn is_on_curve(p: &ProjectivePoint<N, Field, Self>) -> bool {
        let (X, Y, Z) = (p.x, p.y, p.z);
        let (X2, Y2, Z2, XZ) = (X * X, Y * Y, Z * Z, X * Z);
        Y2 + Y * XZ == X2 * XZ + Self::A2 * X2 * Z2 + Self::A6 * (Z2 * Z2)
    }

    // Lopez-Dahab Coordinates based point addition
    fn add(
        lft: &ProjectivePoint<N, Field, Self>,
        rhs: &ProjectivePoint<N, Field, Self>,
    ) -> ProjectivePoint<N, Field, Self> {
        // trivial checks at the very first
        if lft.is_identity() {
            return *rhs;
        }
        if rhs.is_identity() {
            return *lft;
        }
        if lft.neg() == *rhs {
            return Self::IDENTITY;
        }
        let ((X1, Y1, Z1), (X2, Y2, Z2)) = ((lft.x, lft.y, lft.z), (rhs.x, rhs.y, rhs.z));
        // mixed Coordinates
        if rhs.is_affine() {
            let (A, B) = (Y1 + Y2 * (Z1 * Z1), X1 + X2 * Z1);
            let C = B * Z1;
            let Z3 = C * C;
            let D = X2 * Z3;
            let X3 = A * A + C * (A + B * B + Self::A2 * C);
            let Y3 = (D + X3) * (A * C + Z3) + (Y2 + X2) * (Z3 * Z3);
            ProjectivePoint {
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
            ProjectivePoint {
                x: X3,
                y: Y3,
                z: Z3,
                marker: PhantomData::<Self>,
            }
        }
    }

    // Lopez-Dahab coordinates based point doubling
    fn double(lft: &ProjectivePoint<N, Field, Self>) -> ProjectivePoint<N, Field, Self> {
        if lft.is_identity() {
            return Self::IDENTITY;
        }
        let (X1, Y1, Z1) = (lft.x, lft.y, lft.z);
        let A = Z1 * Z1;
        let (B, C) = (Self::A6 * (A * A), X1 * X1);
        let Z3 = A * C;
        let X3 = C * C + B;
        let Y3 = (Y1 * Y1 + Self::A2 * Z3 + B) * X3 + Z3 * B;
        ProjectivePoint {
            x: X3,
            y: Y3,
            z: Z3,
            marker: PhantomData::<Self>,
        }
    }

    // two solutions for y when x is fixed, i.e two points which are negative with each other
    fn neg(lft: &ProjectivePoint<N, Field, Self>) -> ProjectivePoint<N, Field, Self> {
        if lft.is_identity() {
            return Self::IDENTITY;
        }
        ProjectivePoint {
            x: lft.x,
            y: lft.y + lft.x * lft.z,
            z: lft.z,
            marker: PhantomData::<Self>,
        }
    }

    // General Montgomery scalar multiplication
    fn montgomery_scalar_mul(
        p: &ProjectivePoint<N, Field, Self>,
        scalar: Field,
    ) -> ProjectivePoint<N, Field, Self> {
        // trivial checks at the very first
        if scalar.is_zero() {
            return Self::IDENTITY;
        }
        if scalar.is_one() {
            return *p;
        }
        if p.is_identity() {
            return Self::IDENTITY;
        }
        // binary representation of scalar field
        let n = scalar.bits(true);
        let l = n.len();
        let (mut P1, mut P2) = (p.clone(), *p + *p);
        // using montgomery ladder
        for i in (0..l - 1).rev() {
            if n[i] == 0u8 {
                // P1 = P1 + P1, P2 = P1 + P2
                (P1, P2) = (P1 + P1, P1 + P2);
            } else {
                // P1 = P1 + P2, P2 = P2 + P2
                (P1, P2) = (P1 + P2, P2 + P2);
            }
        }
        P1
    }

    // Z_m+n = (XnZm)^2 + (XmZn)^2
    // X_m+n = Z_m+n * X_m-n + (XnZm) * (Xm * Zn)
    fn mont_double(pn: &(Field, Field)) -> (Field, Field) {
        let (Xn, Zn) = (pn.0, pn.1);
        let (Xn_sq, Zn_sq) = (Xn.squaring(), Zn.squaring());
        ((Xn_sq + Self::A6_SQRT * Zn_sq).squaring(), Xn_sq * Zn_sq)
    }

    // X_2n = (X_n^2 + \sqrt(a_6) * Z_n^2)^2
    // Z_2n = X_n^2 * Z_n^2
    fn mont_add(pn: &(Field, Field), pm: &(Field, Field), X_m_minus_n: Field) -> (Field, Field) {
        let ((Xn, Zn), (Xm, Zm)) = ((pn.0, pn.1), (pm.0, pm.1));
        let (XmZn, XnZm) = (Xm * Zn, Xn * Zm);
        let Z_m_plus_n = (XmZn + XnZm).squaring();
        (Z_m_plus_n * X_m_minus_n + XmZn * XnZm, Z_m_plus_n)
    }

    // Fast Montgomery scalar multiplication specially for binary curve
    fn fast_montgomery_scalar_mul(
        p: &ProjectivePoint<N, Field, Self>,
        scalar: Field,
    ) -> ProjectivePoint<N, Field, Self> {
        // trivial checks at the very first
        if scalar.is_zero() {
            return Self::IDENTITY;
        }
        if scalar.is_one() {
            return *p;
        }
        if p.is_identity() {
            return Self::IDENTITY;
        }
        // binary representation of scalar field
        let n = scalar.bits(true);
        let l = n.len();
        // P1, P2
        let ((mut Xn, mut Zn), (mut Xm, mut Zm)) = ((p.x, p.z), Self::mont_double(&(p.x, p.z)));
        // P = P2 - P1
        let (Xm_minus_n, Ym_minus_n) = (p.x, p.y);
        // using montgomery ladder
        for i in (0..l - 1).rev() {
            if n[i] == 0u8 {
                // P1 = P1 + P1, P2 = P1 + P2
                ((Xn, Zn), (Xm, Zm)) = (
                    Self::mont_double(&(Xn, Zn)),
                    Self::mont_add(&(Xn, Zn), &(Xm, Zm), Xm_minus_n),
                );
            } else {
                // P1 = P1 + P2, P2 = P2 + P2
                ((Xn, Zn), (Xm, Zm)) = (
                    Self::mont_add(&(Xn, Zn), &(Xm, Zm), Xm_minus_n),
                    Self::mont_double(&(Xm, Zm)),
                );
            }
        }
        // convert to affine coordinates
        ((Xn, Zn), (Xm, _)) = ((Xn / Zn, Field::one()), (Xm / Zm, Field::one()));
        // restore Yn with restored affine coordinates of [n]P and [m]P
        // Y_n = (Xn + Xm_minus_n) * ((Xn + Xm_minus_n) * (Xm + Xm_minus_n) + Xm_minus_n^2 + Ym_minus_n) / Xm_minus_n + Ym_minus_n
        let Xn_Plus_Xm_minus_n = Xn + Xm_minus_n;
        let Yn = (Xn_Plus_Xm_minus_n
            * (Xn_Plus_Xm_minus_n * (Xm + Xm_minus_n) + Xm_minus_n.squaring() + Ym_minus_n))
            / Xm_minus_n
            + Ym_minus_n;
        ProjectivePoint {
            x: Xn,
            y: Yn,
            z: Zn,
            marker: PhantomData::<Self>,
        }
    }
}
