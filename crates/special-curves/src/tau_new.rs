use crate::rational_quadratic::{RationalQuadraticField, Round};
use core::ops::{Add, Div, Mul, Sub};
use num_rational::Rational64;

type BaseField = i64;
pub const MU: BaseField = -1;

pub trait Norm {
    type Output;
    fn norm(&self) -> Self::Output;
}

// Trait for convert a complex number to its conjugate number, or vice versa
pub trait ToConjugate {
    type Output;
    fn to_conjugate(&self) -> Self::Output;
}

#[derive(Clone)]
pub struct IntegerQuadraticField {
    pub a0: BaseField,
    pub a1: BaseField,
}

impl Add for IntegerQuadraticField {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            a0: self.a0 + other.a0,
            a1: self.a1 + other.a1,
        }
    }
}

impl Sub for IntegerQuadraticField {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            a0: self.a0 - other.a0,
            a1: self.a1 - other.a1,
        }
    }
}
#[derive(Clone)]
pub struct TauComplex(IntegerQuadraticField);
pub struct TauConjugate(IntegerQuadraticField);

impl AsRef<IntegerQuadraticField> for TauComplex {
    fn as_ref(&self) -> &IntegerQuadraticField {
        &self.0
    }
}

impl Sub for TauComplex {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            0: self.0 - other.0,
        }
    }
}

impl Mul<TauConjugate> for TauComplex {
    type Output = Self;

    fn mul(self, rhs: TauConjugate) -> Self::Output {
        let a0 = self.0.a0 * rhs.0.a0 + MU * self.0.a0 * rhs.0.a1 + 2 * self.0.a1 * rhs.0.a1;
        let a1 = self.0.a1 * rhs.0.a0 - self.0.a0 * rhs.0.a1;
        Self {
            0: IntegerQuadraticField::new(a0, a1),
        }
    }
}

impl Norm for IntegerQuadraticField {
    type Output = BaseField;

    fn norm(&self) -> Self::Output {
        let a0 = self.a0 * self.a0 + 2 * self.a1 * self.a1;
        let a1 = MU * self.a0 * self.a1;
        a0 + a1
    }
}

impl Norm for TauComplex {
    type Output = BaseField;

    fn norm(&self) -> Self::Output {
        self.0.norm()
    }
}

impl ToConjugate for TauComplex {
    type Output = TauConjugate;

    fn to_conjugate(&self) -> Self::Output {
        self.0.to_conjugate().as_conjugate()
    }
}

impl Div<Self> for TauComplex {
    type Output = (Self, Self);

    fn div(self, rhs: Self) -> Self::Output {
        let g = self.clone() * rhs.conjugate();
        let big_n = rhs.norm();
        let k = RationalQuadraticField::new(
            Rational64::new(g.0.a0, big_n),
            Rational64::new(g.0.a1, big_n),
        )
        .round_off()
        .as_complex();
        let ro = self - k.clone() * rhs.to_conjugate();

        (k, ro)
    }
}

impl TauComplex {
    // Returns the conjugate of the complex number
    pub fn conjugate(&self) -> TauConjugate {
        self.0.as_conjugate()
    }
}

impl Sub for TauConjugate {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            0: self.0 - other.0,
        }
    }
}

impl Mul<TauComplex> for TauConjugate {
    type Output = Self;

    fn mul(self, rhs: TauComplex) -> Self::Output {
        let a0 = self.0.a0 * rhs.0.a0 + MU * self.0.a0 * rhs.0.a1 + 2 * self.0.a1 * rhs.0.a1;
        let a1 = self.0.a1 * rhs.0.a0 - self.0.a0 * rhs.0.a1;
        Self {
            0: IntegerQuadraticField::new(a0, a1),
        }
    }
}

impl Norm for TauConjugate {
    type Output = BaseField;

    fn norm(&self) -> Self::Output {
        self.0.norm()
    }
}

impl ToConjugate for TauConjugate {
    type Output = TauComplex;

    fn to_conjugate(&self) -> Self::Output {
        self.0.to_conjugate().as_complex()
    }
}

impl AsRef<IntegerQuadraticField> for TauConjugate {
    fn as_ref(&self) -> &IntegerQuadraticField {
        &self.0
    }
}

// Constructor functions
impl IntegerQuadraticField {
    pub fn new(a0: BaseField, a1: BaseField) -> Self {
        Self { a0, a1 }
    }
    pub fn as_complex(&self) -> TauComplex {
        TauComplex(self.clone())
    }

    pub fn as_conjugate(&self) -> TauConjugate {
        TauConjugate(self.clone())
    }
}

impl ToConjugate for IntegerQuadraticField {
    type Output = Self;

    fn to_conjugate(&self) -> Self::Output {
        Self {
            a0: self.a0 + MU * self.a1,
            a1: -self.a1,
        }
    }
}
