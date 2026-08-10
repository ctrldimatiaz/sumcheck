use std::fmt::Display;

use itertools::Itertools;
use log::{debug, error};

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
        if !values
            .iter()
            .map(|m| m.get_number_of_variables())
            .all_equal()
        {
            return Err(PolynomialError::DifferentVariableCounts);
        }

        Ok(Self { terms: values })
    }

    // Check if polynomial is multilinear by checking that all terms are multilinear
    pub fn is_multilinear(&self) -> bool {
        self.terms.iter().all(|term| term.is_multilinear())
    }

    // All monomials must be constant. After we add collect terms in monomial this function will
    // work properlly without any chance of different terms nulling each other.
    pub fn is_constant(&self) -> bool {
        self.terms.iter().all(|term| term.is_constant())
    }

    // Reusable function to print the readable polynomial
    pub fn get_readable_polynomial(&self) -> String {
        self.terms.iter().join(" + ")
    }

    // Evaluate the polynomial with the provided values.
    pub fn evaluate(
        &self,
        values: &Vec<FieldElement<P>>,
    ) -> Result<FieldElement<P>, PolynomialError> {
        // We must check against the right number of values
        //
        debug!(
            "Evaluation of polynomial {} with terms of term {} with values {}",
            self,
            self.terms
                .iter()
                .map(|m| m.get_number_of_variables())
                .join(" , "),
            values.iter().join(" ")
        );
        if self
            .terms
            .iter()
            .any(|t| t.get_number_of_variables() != values.len())
        {
            return Err(PolynomialError::DifferentVariableCounts);
        }

        let mut result: FieldElement<P> = FieldElement::zero();

        // go through all the monomials and perform the individual evaluation
        for term in &self.terms {
            let term_evaluation_result = term.evaluate(values);

            if term_evaluation_result.is_err() {
                error!(
                    "Failed to evaluate monomial values: {}",
                    term_evaluation_result.unwrap_err()
                );
                return term_evaluation_result;
            }

            result = result + term_evaluation_result.unwrap();
        }

        Ok(result)
    }

    // Reduce the polynomial to the variable derived from fixed_terms
    // [] -> reduce to x1. [1] -> x2 ...
    pub fn reduce_polynomial(
        &self,
        fixed_terms: &Vec<FieldElement<P>>,
    ) -> Result<Polynomial<P>, PolynomialError> {
        let mut monomials_evaluated: Vec<Monomial<P>> = vec![];

        let no_of_terms_not_fixed = self.terms.len() - (fixed_terms.len() + 1);

        let number_of_combinations = 2_u64.pow(no_of_terms_not_fixed as u32);

        for i in 0..number_of_combinations {
            let mut values: Vec<FieldElement<P>> = fixed_terms.clone();

            values.extend(
                number_to_bits_vec(i, no_of_terms_not_fixed)
                    .iter()
                    .map(|bit| FieldElement::from_u64(*bit)),
            );

            for monomial in &self.terms {
                monomials_evaluated
                    .push(monomial.evaluate_with_fixing_term(fixed_terms.len() as u64, &values));
            }
        }

        let poly = Polynomial::new(monomials_evaluated).unwrap();

        debug!(
            "Polynomial reduced {} with terms {}",
            poly,
            poly.get_number_of_variables()
        );

        Ok(poly)
    }

    // Compute the sum through the hyperbolic hypercube
    pub fn compute_sum(&self) -> FieldElement<P> {
        let mut result = FieldElement::zero();
        let number_of_combinations = 2_u64.pow(self.terms.len() as u32);

        // evaluate through all the combinations according to the number of variables
        // ex.: 3 variables would go through 0 - (0,0,0), 1 - (0,0,1)... and would have 2³ = 8 combinations
        for i in 0..number_of_combinations {
            let values = number_to_bits_vec(i, self.terms.len())
                .iter()
                .map(|bit| FieldElement::from_u64(*bit))
                .collect();

            result = result + self.evaluate(&values).unwrap();
        }

        result
    }

    // Get the numebr of variables of the polynomial. We are sure all the variables are referenced
    // doing initialization and expect the error DifferentVariableCount otherwise
    pub fn get_number_of_variables(&self) -> usize {
        self.terms.first().unwrap().get_number_of_variables()
    }
}

// Display
impl<const P: u64> Display for Polynomial<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_readable_polynomial())
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
