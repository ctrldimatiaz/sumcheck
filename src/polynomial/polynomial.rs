use itertools::Itertools;

use crate::{
    error::PolynomialError, field::field_element::FieldElement, polynomial::monomial::Monomial,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Polynomial<const P: u64> {
    terms: Vec<Monomial<P>>,
}

impl<const P: u64> Polynomial<P> {
    pub fn new(values: Vec<Monomial<P>>) -> Result<Self, PolynomialError> {
        if values.is_empty() {
            return Err(PolynomialError::EmptyPolynomial);
        }

        if !values.iter().map(|m| m.get_number_of_terms()).all_equal() {
            return Err(PolynomialError::DifferentVariableCounts);
        }

        Ok(Self { terms: values })
    }

    pub fn is_multilinear(&self) -> bool {
        self.terms.iter().any(|term| term.is_multilinear())
    }

    pub fn is_constant(&self) -> bool {
        self.terms.iter().all(|term| term.is_constant())
    }

    pub fn get_readable_polynomial(&self) -> String {
        self.terms.iter().join(" + ")
    }

    pub fn evaluate(
        &self,
        values: Vec<FieldElement<P>>,
    ) -> Result<FieldElement<P>, PolynomialError> {
        //this must be reviewed because we might have monomials with different number of
        //Ex.: 5x1x2 + 7x1x3x5
        if self
            .terms
            .iter()
            .any(|t| t.get_number_of_terms() != values.len())
        {
            return Err(PolynomialError::DifferentVariableCounts);
        }

        let mut result: FieldElement<P> = FieldElement::zero();

        for term in &self.terms {
            result = result + term.evaluate(&values);
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type P17 = Polynomial<17>;

    #[test]
    fn test_empty_polynomial() {
        let polynomial = P17::new(vec![]).unwrap_err();

        assert_eq!(polynomial, PolynomialError::EmptyPolynomial);
    }

    #[test]
    fn test_polynomial_constantness() {
        let zerocoeff_constant_polynomial = P17::new(vec![
            Monomial::new(FieldElement::from_u64(5), vec![0, 0]),
            Monomial::new(FieldElement::from_u64(0), vec![1, 1]),
        ])
        .unwrap();

        let constant_polynomial = P17::new(vec![
            Monomial::new(FieldElement::from_u64(5), vec![0, 0]),
            Monomial::new(FieldElement::from_u64(10), vec![0, 0]),
        ])
        .unwrap();

        assert!(constant_polynomial.is_constant());
        assert!(zerocoeff_constant_polynomial.is_constant());
    }
}
