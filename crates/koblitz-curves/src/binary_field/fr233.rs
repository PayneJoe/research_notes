use super::{BinaryPolynomial, N};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Fq233(pub BinaryPolynomial<N>);
