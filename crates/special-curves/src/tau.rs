use core::ops::Sub;
use num_rational::Rational64;

pub trait Round {
    fn round_off(&self) -> Self;
}

impl Round for Rational64 {
    fn round_off(&self) -> Self {
        let half = Rational64::new(1, 2);
        if self.numer().signum() * self.denom().signum() > 0 {
            (self.clone() - half).ceil()
        } else {
            (self.clone() + half).floor()
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Tau {
    lambda0: Rational64,
    lambda1: Rational64,
}

impl Sub for Tau {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            lambda0: self.lambda0 - other.lambda0,
            lambda1: self.lambda1 - other.lambda1,
        }
    }
}

impl Tau {
    fn new(lambda0: Rational64, lambda1: Rational64) -> Self {
        Self { lambda0, lambda1 }
    }

    fn norm(&self) -> Rational64 {
        // N(lambda) = lambda0^2 + lambda0 * lambda1 * (\tau + \bar{\tau}) + lambda1^2 * \tau * \bar{\tau}
        //           = lambda0^2 + lambda0 * lambda1 * \mu + 2 * lambda1^2
        let (lambda0_square, lambda0_mul_lambda1, lambda1_square) = (
            self.lambda0 * self.lambda0,
            self.lambda0 * self.lambda1,
            self.lambda1 * self.lambda1,
        );
        let MU: Rational64 = -Rational64::ONE;
        lambda0_square + lambda0_mul_lambda1 * MU + Rational64::from_integer(2) * lambda1_square
    }
}

impl Round for Tau {
    // algorithm 15.9 of "handbook of elliptic and hyperelliptic curve cryptography"
    // http://pustaka.unp.ac.id/file/abstrak_kki/EBOOKS/Kriptografi%20dan%20Ethical%20Hacking%20B.pdf
    fn round_off(&self) -> Self {
        // curve function: y^2 + xy = x^3 + a2 x^2 + 1, with as = 0
        let MU: Rational64 = -Rational64::ONE;
        let (f0, f1) = (self.lambda0.round_off(), self.lambda1.round_off());
        println!("f0 = {:?}, f1 = {:?}", f0, f1);
        let (eta0, eta1) = (self.lambda0 - f0, self.lambda1 - f1);
        let (mut h0, mut h1) = (Rational64::from_integer(0), Rational64::from_integer(0));
        let (ONE, TWO, THREE, FOUR) = (
            Rational64::ONE,
            Rational64::from_integer(2),
            Rational64::from_integer(3),
            Rational64::from_integer(4),
        );
        let eta = TWO * eta0 + MU * eta1;
        if eta >= ONE {
            if eta0 - THREE * MU * eta1 < -ONE {
                h1 = MU;
            } else {
                h0 = ONE;
            }
        } else {
            if eta0 + FOUR * MU * eta1 >= TWO {
                h1 = MU;
            }
        }
        if eta < -ONE {
            if eta0 - THREE * MU * eta1 >= ONE {
                h1 = -MU;
            } else {
                h0 = -ONE;
            }
        } else {
            if eta0 + FOUR * MU * eta1 < -TWO {
                h1 = -MU;
            }
        }
        let (q0, q1) = (f0 + h0, f1 + h1);
        Self {
            lambda0: q0,
            lambda1: q1,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_rational_round_off() {
        let r1 = Rational64::new(1, 2);
        assert_eq!(r1.round_off(), Rational64::new(0, 1), "Round(1/2) == 0");
        let r2 = Rational64::new(-1, 2);
        assert_eq!(r2.round_off(), Rational64::new(0, 1), "Round(-1/2) == 0");
    }

    #[test]
    fn test_lambda_round_off() {
        let (lambda0, lambda1) = (Rational64::new(8, 5), Rational64::new(12, 5));
        let lambda = Tau::new(lambda0, lambda1);
        let lambda_ro = lambda.round_off();
        assert_eq!(
            lambda_ro,
            Tau::new(Rational64::from_integer(1), Rational64::from_integer(2))
        );
        let diff = (lambda - lambda_ro).norm();
        let diff_base =
            (lambda - Tau::new(Rational64::from_integer(2), Rational64::from_integer(2))).norm();
        assert!(diff < diff_base, "Closest lattice element");
    }
}
