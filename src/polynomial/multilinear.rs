use std::fmt::Display;

use itertools::Itertools;

use crate::{
    error::PolynomialError, field::field_element::FieldElement, polynomial::polynomial::Polynomial,
};

pub struct MultilinearPolynomial<const P: u64> {
    polynomial: Polynomial<P>,
}

impl<const P: u64> MultilinearPolynomial<P> {
    pub fn new(polynomial: Polynomial<P>) -> Result<Self, PolynomialError> {
        for term in &polynomial.terms {
            if !term.is_multilinear() {
                return Err(PolynomialError::NotMultilinear);
            }
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
        write!(f, "{}", self.polynomial.terms.iter().join(" + "))
    }
}
