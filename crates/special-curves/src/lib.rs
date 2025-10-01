pub mod integer_quadratic;
pub mod rational_quadratic;
pub mod tau;
pub mod tau_adic;

//////////////////// Traits
pub trait AsRational {
    type Output;
    fn as_rational(&self) -> Self::Output;
}

pub trait AsInteger {
    type Output;
    fn as_integer(&self) -> Self::Output;
}

pub trait Norm {
    type Output;
    fn norm(&self) -> Self::Output;
}

pub trait Round {
    type Output;
    fn round_off(&self) -> Self::Output;
}

pub trait Pow {
    type Output;
    fn pow(&self, e: i64) -> Self;
}

pub trait LucasSequence {
    type Output;
    fn next(&self) -> Self;
}
