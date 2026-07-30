use std::fmt::Display;

use crate::{
    error::PolynomialError, field::field_element::FieldElement, polynomial::polynomial::Polynomial,
};

pub struct MultilinearPolynomial<const P: u64> {
    polynomial: Polynomial<P>,
}

impl<const P: u64> MultilinearPolynomial<P> {
    pub fn new(polynomial: Polynomial<P>) -> Result<Self, PolynomialError> {
        if polynomial.is_multilinear() {
            return Err(PolynomialError::NotMultilinear);
        }

        if polynomial.is_constant() {
            return Err(PolynomialError::ConstantPolynomial);
        }

        Ok(Self { polynomial })
    }

    pub fn evaluate(
        &self,
        values: Vec<FieldElement<P>>,
    ) -> Result<FieldElement<P>, PolynomialError> {
        self.polynomial.evaluate(values)
    }
}

// Display
impl<const P: u64> Display for MultilinearPolynomial<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.polynomial.get_readable_polynomial())
    }
}
