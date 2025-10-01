use crate::AsInteger;
// use crate::LucasSequence;
use crate::Pow;
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

type TauLucasSequence = LucasSequence<0, 1>;

#[derive(Copy, Clone, Debug)]
pub struct Tau(IntegerQuadraticField);

impl Tau {
    pub fn pow(&self, e: usize) -> Self {
        if e == 0 {
            return Self(IntegerQuadraticField::one());
        }
        let mut result = TauLucasSequence::new();
        todo!()
    }
}

// #[derive(Clone, Copy, Debug, Eq, PartialEq)]
// pub struct Tau {
//     u0: IntegerBaseField,
//     u1: IntegerBaseField,
// }
//
// impl LucasSequence for Tau {
//     type Output = Self;
//     // refer to "Handbook of Elliptic and Hyperelliptic Curve Cryptography", equation 15.4
//     fn next(&self) -> Self::Output {
//         Self {
//             u0: self.u1,
//             u1: MU * self.u1 - BIAS * self.u0,
//         }
//     }
// }
//
// impl AsInteger for Tau {
//     type Output = IntegerQuadraticField;
//     fn as_integer(&self) -> Self::Output {
//         IntegerQuadraticField::new(-BIAS * self.u0, self.u1)
//     }
// }
//
// impl Pow for Tau {
//     type Output = Self;
//     fn pow(&self, e: i64) -> Self::Output {
//         assert!(e >= 1, "e must be greater or equal than 1");
//         let mut result = Self { u0: 0, u1: 1 };
//         for _ in 1..e {
//             result = result.next();
//         }
//         result
//     }
// }
