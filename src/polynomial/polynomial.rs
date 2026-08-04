use itertools::Itertools;

use crate::{
    field::field_element::FieldElement,
    helpers::{error::PolynomialError, functions::number_to_bits_vec},
    polynomial::monomial::Monomial,
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

        // the terms must all be explicitly defined during initialization
        // Ex.: 5x1x2 + 7x1x3x5
        // 5[1,1,0,0,0] and 7[1,0,1,0,1]
        if !values.iter().map(|m| m.get_number_of_terms()).all_equal() {
            return Err(PolynomialError::DifferentVariableCounts);
        }

        Ok(Self { terms: values })
    }

    pub fn is_multilinear(&self) -> bool {
        self.terms.iter().all(|term| term.is_multilinear())
    }

    pub fn is_constant(&self) -> bool {
        self.terms.iter().all(|term| term.is_constant())
    }

    pub fn get_readable_polynomial(&self) -> String {
        self.terms.iter().join(" + ")
    }

    pub fn evaluate(
        &self,
        values: &Vec<FieldElement<P>>,
    ) -> Result<FieldElement<P>, PolynomialError> {
        // We must check against the right number of values
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

    pub fn reduce_polynomial(&self, round: &Vec<FieldElement<P>>) -> Polynomial<P> {
        let mut reduced_polynomial = Polynomial::new(vec![]).unwrap();

        let no_of_terms_not_fixed = self.terms.len() - (round.len() + 1);

        let number_of_combinations = 2_u64.pow(no_of_terms_not_fixed as u32);

        for i in 0..number_of_combinations {
            let _values = number_to_bits_vec(i, no_of_terms_not_fixed);
        }

        reduced_polynomial
    }

    fn evaluate_with_fixing_term(&self, values: &Vec<FieldElement<P>>) -> Polynomial<P> {
        Polynomial::new(vec![]).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type P17 = Polynomial<17>;

    #[test]
    fn test_empty_polynomial() {
        let polynomial = P17::new(vec![]).unwrap_err();
        let multilinear_polynomial = P17::new(vec![
            Monomial::new(FieldElement::from_u64(5), vec![0, 0]),
            Monomial::new(FieldElement::from_u64(5), vec![1, 1]),
        ]);

        assert!(multilinear_polynomial.is_ok());
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

    #[test]
    fn test_polynomial_multilinearity() {
        let multilinear_polynomial = P17::new(vec![
            Monomial::new(FieldElement::from_u64(5), vec![0, 0]),
            Monomial::new(FieldElement::from_u64(5), vec![1, 1]),
        ])
        .unwrap();

        let not_multilinear_polynomial = P17::new(vec![
            Monomial::new(FieldElement::from_u64(5), vec![0, 4]),
            Monomial::new(FieldElement::from_u64(10), vec![0, 0]),
        ])
        .unwrap();

        assert!(multilinear_polynomial.is_multilinear());
        assert!(!not_multilinear_polynomial.is_multilinear());
    }

    #[test]
    fn test_polynomial_evaluation() {
        //0
        let zero_multilinear_polynomial = P17::new(vec![
            Monomial::new(FieldElement::from_u64(0), vec![0, 0]),
            Monomial::new(FieldElement::from_u64(0), vec![1, 1]),
        ])
        .unwrap();

        //5x2⁴ + 10
        let multilinear_polynomial = P17::new(vec![
            Monomial::new(FieldElement::from_u64(5), vec![0, 4]),
            Monomial::new(FieldElement::from_u64(10), vec![0, 0]),
        ])
        .unwrap();

        let values = vec![FieldElement::from_u64(5u64), FieldElement::from_u64(2u64)];

        assert_eq!(
            zero_multilinear_polynomial.evaluate(&values).unwrap(),
            FieldElement::zero()
        );
        assert_eq!(
            multilinear_polynomial.evaluate(&values).unwrap(),
            FieldElement::from_u64(5u64)
        );
    }
}
