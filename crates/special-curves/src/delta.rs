use num_rational::Rational64;

pub const MU: Rational64 = Rational64::ONE;

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

pub struct Delta {
    delta0: Rational64,
    delta1: Rational64,
}

impl Round for Delta {
    // algorithm 15.9 of "handbook of elliptic and hyperelliptic curve cryptography"
    // http://pustaka.unp.ac.id/file/abstrak_kki/EBOOKS/Kriptografi%20dan%20Ethical%20Hacking%20B.pdf
    fn round_off(&self) -> Self {
        let (f0, f1) = (self.delta0.round_off(), self.delta1.round_off());
        let (eta0, eta1) = (self.delta0 - f0, self.delta1 - f1);
        let (mut h0, mut h1) = (Rational64::from_integer(0), Rational64::from_integer(0));
        let eta = Rational64::from_integer(2) * eta0 + MU * eta1;
        let (ONE, TWO, THREE, FOUR) = (
            Rational64::ONE,
            Rational64::from_integer(2),
            Rational64::from_integer(3),
            Rational64::from_integer(4),
        );
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
            delta0: q0,
            delta1: q1,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_rational() {
        let r1 = Rational64::new(1, 2);
        assert_eq!(r1.round_off(), Rational64::new(0, 1), "Round(1/2) == 0");
        let r2 = Rational64::new(-1, 2);
        assert_eq!(r2.round_off(), Rational64::new(0, 1), "Round(-1/2) == 0");
    }
}
