use num_rational::Rational64;

pub trait Round {
    fn better_round(&self) -> Self;
}

impl Round for Rational64 {
    fn better_round(&self) -> Self {
        let half = Rational64::new(1, 2);
        if self.numer().signum() * self.denom().signum() > 0 {
            (self.clone() - half).ceil()
        } else {
            (self.clone() + half).floor()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_rational() {
        let r1 = Rational64::new(1, 2);
        assert_eq!(r1.better_round(), Rational64::new(0, 1), "Round(1/2) == 0");
        let r2 = Rational64::new(-1, 2);
        assert_eq!(r2.better_round(), Rational64::new(0, 1), "Round(-1/2) == 0");
    }
}
