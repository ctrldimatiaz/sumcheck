use crate::{
    field::field_element::FieldElement, helpers::error::PolynomialError,
    polynomials::multilinear::MultilinearPolynomial,
};

pub struct Oracle<const P: u64> {
    polynomial: MultilinearPolynomial<P>,
}

impl<const P: u64> Oracle<P> {
    pub fn new(polynomial: MultilinearPolynomial<P>) -> Self {
        Oracle { polynomial }
    }

    pub fn evaluate(&self, point: &[FieldElement<P>]) -> Result<FieldElement<P>, PolynomialError> {
        self.polynomial.evaluate(point)
    }
}
