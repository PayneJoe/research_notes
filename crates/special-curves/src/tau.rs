use crate::AsInteger;
use crate::Modulos;
use crate::integer_quadratic::{BIAS, IntegerBaseField, IntegerQuadraticField, MU};

#[derive(Copy, Clone, Debug)]
struct LucasSequence<const U0: IntegerBaseField = 0, const U1: IntegerBaseField = 1> {
    u0: IntegerBaseField,
    u1: IntegerBaseField,
}

impl<const U0: IntegerBaseField, const U1: IntegerBaseField> LucasSequence<U0, U1> {
    const fn new() -> Self {
        Self { u0: U0, u1: U1 }
    }

    // refer to "Handbook of Elliptic and Hyperelliptic Curve Cryptography", equation 15.4
    pub fn next(&self) -> Self {
        Self {
            u0: self.u1,
            u1: MU * self.u1 - BIAS * self.u0,
        }
    }
}

impl<const U0: IntegerBaseField, const U1: IntegerBaseField> AsInteger for LucasSequence<U0, U1> {
    type Output = IntegerQuadraticField;
    fn as_integer(&self) -> Self::Output {
        IntegerQuadraticField::new(-BIAS * self.u0, self.u1)
    }
}

type TauLucasSequence = LucasSequence<0, 1>;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Tau(pub IntegerQuadraticField);

impl Default for Tau {
    fn default() -> Self {
        Self(TauLucasSequence::new().as_integer())
    }
}

impl Tau {
    pub fn trace(&self) -> IntegerBaseField {
        MU
    }

    pub fn determint(&self) -> IntegerBaseField {
        BIAS
    }

    pub fn value(&self) -> &IntegerQuadraticField {
        &self.0
    }
    // \tau^d = U_d * \tau - U_{d - 1} * BIAS
    pub fn pow(&self, d: usize) -> Self {
        if d == 0 {
            return Self(IntegerQuadraticField::one());
        }
        if d == 1 {
            return *self;
        }
        let mut result = TauLucasSequence::new();
        for _ in 1..d {
            result = result.next();
        }

        Self(result.as_integer())
    }

    // convert to \tau-NAF representation
    // refer to "Handbook of Elliptic and Hyperelliptic Curve Cryptography", Algorithm 15.6
    pub fn to_naf(&self) -> Vec<i8> {
        let mut s = vec![];
        let (mut n0, mut n1) = (self.value().a0, self.value().a1);
        let mut r: i8;
        while n0.abs() + n1.abs() != 0 {
            if n0.modulos(2) == 1 {
                r = (2 - (n0 - 2 * n1).modulos(4)) as i8;
                n0 -= r as i64;
            } else {
                r = 0;
            }
            s.push(r);
            (n0, n1) = (n1 + n0 / 2, -n0 / 2);
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_tau_pow() {
        let tau = Tau::default();
        assert_eq!(
            *tau.pow(2).value(),
            IntegerQuadraticField::new(-BIAS * 1, MU)
        )
    }

    #[test]
    fn test_delta() {
        let tau = Tau::default();
        let nominator = *tau.pow(11).value() - IntegerQuadraticField::one();
        let denominator = *tau.value() - IntegerQuadraticField::one();
        let (quotient, remainder) = nominator / denominator;
        assert_eq!(quotient, IntegerQuadraticField::new(23, -22));
        assert_eq!(remainder, IntegerQuadraticField::zero());
    }

    // refer to "Handbook of Elliptic and Hyperelliptic Curve Cryptography", Example 15.8
    #[test]
    fn test_to_tau_naf() {
        let scalar = Tau(IntegerQuadraticField::new(409, 0));
        let tau_naf = scalar.to_naf();
        let expected_tau_naf = vec![1, 0, 0, 1, 0, 0, 1, 0, -1, 0, 1, 0, 0, 0, 0, -1, 0, 0, -1];
        assert_eq!(tau_naf, expected_tau_naf);
    }
}
