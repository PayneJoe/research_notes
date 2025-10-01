use crate::integer_quadratic::{BIAS, IntegerQuadraticField, MU};
use crate::{AsRational, Norm, Round};
use core::ops::Sub;
use num_rational::Rational64;

type BaseField = Rational64;

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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RationalQuadraticField {
    pub real: Rational64,
    pub imag: Rational64,
}

impl RationalQuadraticField {
    pub fn new(real: Rational64, imag: Rational64) -> Self {
        Self { real, imag }
    }
}

impl Sub for RationalQuadraticField {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self {
            real: self.real - other.real,
            imag: self.imag - other.imag,
        }
    }
}

impl Norm for RationalQuadraticField {
    type Output = BaseField;
    fn norm(&self) -> Self::Output {
        let (real_square, real_mul_imag, imag_square) = (
            self.real * self.real,
            self.real * self.imag,
            self.imag * self.imag,
        );
        real_square + real_mul_imag * MU + BIAS.as_rational() * imag_square
    }
}

impl Round for RationalQuadraticField {
    type Output = IntegerQuadraticField;

    // refer to "handbook of elliptic and hyperelliptic curve cryptography", Algorithm 15.9
    fn round_off(&self) -> Self::Output {
        // curve function: y^2 + xy = x^3 + a_2 * x^2 + 1, with a_2 = 0
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::AsRational;
    #[test]
    fn test_rational_round_off() {
        let r1 = Rational64::new(1, 2);
        assert_eq!(r1.round_off(), 0i64, "Round(1/2) == 0");
        let r2 = Rational64::new(-1, 2);
        assert_eq!(r2.round_off(), 0i64, "Round(-1/2) == 0");
    }

    #[test]
    fn test_lambda_round_off() {
        let (lambda0, lambda1) = (Rational64::new(8, 5), Rational64::new(12, 5));
        let lambda = RationalQuadraticField::new(lambda0, lambda1);
        let lambda_ro = lambda.round_off();
        assert_eq!(lambda_ro, IntegerQuadraticField::new(1, 2));
        let diff = (lambda - lambda_ro.as_rational()).norm();
        let diff_base =
            (lambda - RationalQuadraticField::new(2i64.as_rational(), 2i64.as_rational())).norm();
        assert!(diff < diff_base, "Closest lattice element");
    }
}
