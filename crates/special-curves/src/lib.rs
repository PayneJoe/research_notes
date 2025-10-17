pub mod integer_quadratic;
pub mod rational_quadratic;
pub mod scalar_field;
pub mod tau;

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

// Modulos integers to positive remainder
pub trait Modulos: Sized {
    fn modulos(&self, modulus: Self) -> Self;
}
