use crate::Norm;
use crate::integer_quadratic::{IntegerBaseField, IntegerQuadraticField};
use crate::tau::Tau;

#[derive(Clone, Copy, Eq, PartialEq, Debug, Default)]
pub struct ScalarField(IntegerBaseField);
// The modulus is \tau^11 - 1 / (\tau - 1) = 23 - 22 * \tau
pub const DELTA: Tau = Tau(IntegerQuadraticField { a0: 23, a1: -22 });

impl ScalarField {
    pub fn new(value: IntegerBaseField) -> Self {
        Self(value)
    }

    pub fn value(&self) -> IntegerBaseField {
        self.0
    }

    // Reduce the scalar field element modulo DELTA
    // refer to "Handbook of Elliptic and Hyperelliptic Curve Cryptography", Algorithm 15.13
    pub fn reduce(&self) -> Tau {
        assert!(self.0 < DELTA.value().norm());
        let (_, ro) = IntegerQuadraticField::new(self.0, 0) / *DELTA.value();
        Tau(ro)
    }
}

#[cfg(test)]
mod tests {
    // refer to "Handbook of Elliptic and Hyperelliptic Curve Cryptography", Example 15.15
    #[test]
    fn test_scalar_field_reduce() {
        use crate::integer_quadratic::IntegerQuadraticField;
        use crate::scalar_field::ScalarField;
        use crate::tau::Tau;

        let scalar = ScalarField::new(409);
        let reduced = scalar.reduce();
        let expected = Tau(IntegerQuadraticField::new(13, -9));
        assert_eq!(reduced, expected);
    }
}
