use crate::tau_new::{IntegerQuadraticField, MU};
use num_rational::Rational64;

/// Trait for rounding off a rational number to the nearest integer
pub trait Round {
    type Output;
    fn round_off(&self) -> Self::Output;
}

impl Round for Rational64 {
    type Output = i64;
    fn round_off(&self) -> Self::Output {
        let half = Rational64::new(1, 2);
        if self.numer().signum() * self.denom().signum() > 0 {
            ((self.clone() - half).ceil()).to_integer()
        } else {
            (self.clone() + half).floor().to_integer()
        }
    }
}

pub struct RationalQuadraticField {
    pub real: Rational64,
    pub imag: Rational64,
}

impl RationalQuadraticField {
    pub fn new(real: Rational64, imag: Rational64) -> Self {
        Self { real, imag }
    }
}

impl Round for RationalQuadraticField {
    type Output = IntegerQuadraticField;

    fn round_off(&self) -> Self::Output {
        // curve function: y^2 + xy = x^3 + a2 x^2 + 1, with as = 0
        let (f0, f1) = (self.real.round_off(), self.imag.round_off());
        let (eta0, eta1) = (self.real - f0, self.imag - f1);
        let (mut h0, mut h1) = (0i64, 0i64);
        let (one, two, three, four) = (
            Rational64::ONE,
            Rational64::from_integer(2),
            Rational64::from_integer(3),
            Rational64::from_integer(4),
        );
        let eta = two * eta0 + eta1 * MU;
        if eta >= one {
            if eta0 - three * MU * eta1 < -one {
                h1 = MU;
            } else {
                h0 = 1;
            }
        } else {
            if eta0 + four * MU * eta1 >= two {
                h1 = MU;
            }
        }
        if eta < -one {
            if eta0 - three * MU * eta1 >= one {
                h1 = -MU;
            } else {
                h0 = -1;
            }
        } else {
            if eta0 + four * MU * eta1 < -two {
                h1 = -MU;
            }
        }
        let (q0, q1) = (f0 + h0, f1 + h1);
        IntegerQuadraticField { a0: q0, a1: q1 }
    }
}
