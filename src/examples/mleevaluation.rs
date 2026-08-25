use crate::{
    helpers::{error::PolynomialError, mleinput::MleInput},
    polynomials::multilinear::MultilinearPolynomial,
};

pub struct MultilinearExtensionFromEvaluations<const P: u64> {
    input: MleInput<P>,
}

impl<'a, const P: u64> MultilinearExtensionFromEvaluations<P> {
    pub fn new(input: MleInput<P>) -> Self {
        Self { input }
    }

    // Here we generate f tilde according to a given polynomial
    pub fn generate_f_tilde(&self) -> Result<MultilinearPolynomial<P>, PolynomialError> {
        Err(PolynomialError::EmptyPolynomial)
    }
}
