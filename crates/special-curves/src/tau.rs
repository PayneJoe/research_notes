use crate::AsInteger;
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

#[derive(Copy, Clone, Debug)]
pub struct Tau(IntegerQuadraticField);

impl Default for Tau {
    fn default() -> Self {
        Self(IntegerQuadraticField::new(0, 1))
    }
}

impl Tau {
    pub fn value(&self) -> &IntegerQuadraticField {
        &self.0
    }
    pub fn pow(&self, e: usize) -> Self {
        if e == 0 {
            return Self(IntegerQuadraticField::one());
        }
        if e == 1 {
            return *self;
        }
        let mut result = TauLucasSequence::new();
        for _ in 1..e {
            result = result.next();
        }

        Self(result.as_integer())
    }
}
