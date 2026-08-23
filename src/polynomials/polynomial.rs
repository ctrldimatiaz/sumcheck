use std::{
    collections::HashMap,
    fmt::Display,
    ops::{Add, Mul},
};

use itertools::Itertools;
use log::debug;

use crate::{
    field::field_element::FieldElement,
    helpers::{error::PolynomialError, functions::number_to_bits_vec},
    polynomials::{monomial::Monomial, multilinear::MultilinearPolynomial},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Polynomial<const P: u64> {
    terms: Vec<Monomial<P>>,
}

impl<const P: u64> Polynomial<P> {
    pub fn new(terms: Vec<Monomial<P>>) -> Result<Self, PolynomialError> {
        if terms.is_empty() {
            return Err(PolynomialError::EmptyPolynomial);
        }

        // I assume it is better to simplify before checking the number of variables in case of
        // even when having wrong terms it may null out the wrong ones
        let collected_terms = Polynomial::collect_terms(&terms);

        // the terms must all be explicitly defined during initialization
        // Ex.: 5x1x2 + 7x1x3x5
        // 5[1,1,0,0,0] and 7[1,0,1,0,1]
        if !collected_terms
            .iter()
            .map(|m| m.get_number_of_variables())
            .all_equal()
        {
            return Err(PolynomialError::DifferentVariableCounts);
        }

        Ok(Self {
            terms: collected_terms,
        })
    }

    pub fn constant(coeff: FieldElement<P>, num_of_variables: usize) -> Self {
        let terms = vec![Monomial::constant(coeff, num_of_variables)];
        Self { terms }
    }

    // Check if polynomial is multilinear by checking that all terms multilinearity
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
    pub fn evaluate(&self, values: &[FieldElement<P>]) -> Result<FieldElement<P>, PolynomialError> {
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
            let term_evaluation_result = term.evaluate(values)?;

            result = result + term_evaluation_result;
        }

        Ok(result)
    }

    // Reduce the polynomial to the variable derived from fixed_terms
    // [] -> reduce to x1. [1] -> x2 ...
    pub fn reduce_polynomial(
        &self,
        fixed_terms: &[FieldElement<P>],
    ) -> Result<Polynomial<P>, PolynomialError> {
        let mut monomials_evaluated: Vec<Monomial<P>> = vec![];

        let no_of_terms_not_fixed = self.terms.len() - (fixed_terms.len() + 1);

        let number_of_combinations = 2_u64.pow(no_of_terms_not_fixed as u32);

        for i in 0..number_of_combinations {
            let mut values: Vec<FieldElement<P>> = fixed_terms.to_vec();

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
            let values: Vec<FieldElement<P>> = number_to_bits_vec(i, self.terms.len())
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

    // Get the number of terms (monomials) the polynomial has
    pub fn get_number_of_terms(&self) -> usize {
        self.terms.len()
    }

    // Simplify the polynomial collecting the terms with the same exponents.
    // (=> there are variables repeated that could be simplified)
    fn collect_terms(terms: &[Monomial<P>]) -> Vec<Monomial<P>> {
        let mut map: HashMap<Vec<usize>, FieldElement<P>> = HashMap::new();

        //collect same exponent to a hashmap and sum coefficients
        for term in terms {
            let key = term.exponents().to_vec();
            let entry = map.entry(key).or_default();

            *entry = std::mem::take(entry) + term.coefficient();
        }

        map.into_iter()
            .filter_map(|(exponents, coeff)| {
                if coeff == FieldElement::zero() {
                    return None;
                }

                match Monomial::new(coeff, exponents) {
                    Ok(monomial) => Some(monomial),
                    Err(e) => {
                        debug!("Error creating monomial while collecting terms: {}", e);
                        None
                    }
                }
            })
            .collect()
    }

    fn langrage_basis(values: &[FieldElement<P>]) -> Result<Polynomial<P>, PolynomialError> {
        let size = values.len();

        // now langrage formula
        // 1 - xn or 0 - (1 - xn)
        // evaluation * x1(or (1 -x1))x2(or (1 - x2))...xn( or (1-xn)) as above  ex.: g(0,0) * x1x2 | g(0,1) * x1(1-x2)
        let mut langrage_polynomial: Polynomial<P> =
            Polynomial::constant(FieldElement::one(), size);

        for (i, value) in values.iter().enumerate() {
            //variable to be considered at given index
            let mut variable_monomial = vec![0; size];
            variable_monomial[i] = 1;

            //zero -> ex.: x1
            if *value == FieldElement::<P>::one() {
                let langrage_basis_polynomial =
                    Polynomial::new(vec![Monomial::new(FieldElement::one(), variable_monomial)?])?;
                langrage_polynomial = langrage_polynomial * langrage_basis_polynomial;
            } else {
                //one -> ex.: 1 - x1
                let monomial = Monomial::new(FieldElement::one(), vec![0; size])?;

                let monomial_minus =
                    Monomial::new(FieldElement::from_i64(-1_i64), variable_monomial)?;

                let langrage_basis_polynomial = Polynomial::new(vec![monomial, monomial_minus])?;

                langrage_polynomial = langrage_polynomial * langrage_basis_polynomial;
            }
        }

        Ok(langrage_polynomial)
    }

    // Generate multilinear f tilde from original multilinear polynomial
    pub fn generate_f_tilde(&self) -> Result<MultilinearPolynomial<P>, PolynomialError> {
        if self.is_multilinear() || self.is_constant() {
            return MultilinearPolynomial::new(self.clone());
        }

        let size = self.get_number_of_variables();
        let combinations = 2_u64.pow(size as u32);
        let mut f_tilde: Polynomial<P> = Polynomial::constant(FieldElement::zero(), size);

        // evaluate through all the combinations according to the number of variables
        // ex.: 3 variables would go through 0 - (0,0,0), 1 - (0,0,1)... and would have 2³ = 8 combinations
        for i in 0..combinations {
            let values: Vec<FieldElement<P>> = number_to_bits_vec(i, size)
                .iter()
                .map(|bit| FieldElement::from_u64(*bit))
                .collect();

            let evaluation_result = self.evaluate(&values).unwrap();

            //get langrage polynomial
            let langrage_polynomial =
                Self::langrage_basis(&values)? * Polynomial::constant(evaluation_result, size);

            f_tilde = f_tilde + langrage_polynomial;
        }

        let result = MultilinearPolynomial::new(f_tilde)?;

        Ok(result)
    }
}

// Display
impl<const P: u64> Display for Polynomial<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_readable_polynomial())
    }
}

// Multiply
impl<const P: u64> Mul for Polynomial<P> {
    type Output = Self;
    fn mul(self, polynomial: Polynomial<P>) -> Polynomial<P> {
        let mut final_terms: Vec<Monomial<P>> = vec![];
        for i in 0..self.terms.len() {
            for j in 0..polynomial.get_number_of_terms() {
                let monomial = self.terms[i].clone() * polynomial.terms[j].clone();
                final_terms.push(monomial);
            }
        }
        Self::new(final_terms).unwrap()
    }
}

// Sum polynomials
impl<const P: u64> Add for Polynomial<P> {
    type Output = Self;
    fn add(self, polynomial: Polynomial<P>) -> Polynomial<P> {
        let mut monomials: Vec<Monomial<P>> = self.terms.clone();

        for j in 0..polynomial.get_number_of_terms() {
            monomials.push(polynomial.terms[j].clone());
        }

        Self::new(monomials).unwrap()
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
            Monomial::new(FieldElement::from_u64(5), vec![0, 0]).unwrap(),
            Monomial::new(FieldElement::from_u64(5), vec![1, 1]).unwrap(),
        ]);

        assert!(multilinear_polynomial.is_ok());
        assert_eq!(polynomial, PolynomialError::EmptyPolynomial);
    }

    #[test]
    fn test_polynomial_constantness() {
        let not_constant_polynomial = P17::new(vec![
            Monomial::new(FieldElement::from_u64(5), vec![0, 0]).unwrap(),
            Monomial::new(FieldElement::from_u64(1), vec![1, 1]).unwrap(),
        ])
        .unwrap();

        let constant_polynomial = P17::new(vec![
            Monomial::new(FieldElement::from_u64(5), vec![0, 0]).unwrap(),
            Monomial::new(FieldElement::from_u64(10), vec![0, 0]).unwrap(),
        ])
        .unwrap();

        let constant_polynomial_ctor = P17::constant(FieldElement::from_u64(10), 7);

        assert!(constant_polynomial.is_constant());
        assert!(constant_polynomial_ctor.is_constant());
        assert!(!not_constant_polynomial.is_constant());
    }

    #[test]
    fn test_polynomial_multilinearity() {
        let multilinear_polynomial = P17::new(vec![
            Monomial::new(FieldElement::from_u64(5), vec![0, 0]).unwrap(),
            Monomial::new(FieldElement::from_u64(5), vec![1, 1]).unwrap(),
        ])
        .unwrap();

        let not_multilinear_polynomial = P17::new(vec![
            Monomial::new(FieldElement::from_u64(5), vec![0, 4]).unwrap(),
            Monomial::new(FieldElement::from_u64(10), vec![0, 0]).unwrap(),
        ])
        .unwrap();

        assert!(multilinear_polynomial.is_multilinear());
        assert!(!not_multilinear_polynomial.is_multilinear());
    }

    #[test]
    fn test_polynomial_evaluation() {
        // constant
        let constant_polynomial = P17::new(vec![
            Monomial::new(FieldElement::from_u64(1), vec![0, 0]).unwrap(),
            Monomial::new(FieldElement::from_u64(18), vec![0, 0]).unwrap(),
        ])
        .unwrap();

        //5x2 +10x1x2
        let multilinear_polynomial = P17::new(vec![
            Monomial::new(FieldElement::from_u64(5), vec![0, 1]).unwrap(),
            Monomial::new(FieldElement::from_u64(10), vec![1, 1]).unwrap(),
        ])
        .unwrap();

        let values = vec![FieldElement::from_u64(5u64), FieldElement::from_u64(2u64)];

        assert_eq!(
            constant_polynomial.evaluate(&values).unwrap(),
            FieldElement::from_i64(2_i64)
        );
        assert_eq!(
            multilinear_polynomial.evaluate(&values).unwrap(),
            FieldElement::from_u64(8_u64)
        );
        assert_eq!(
            multilinear_polynomial
                .evaluate(&[FieldElement::zero(); 2])
                .unwrap(),
            FieldElement::zero()
        )
    }

    #[test]
    fn test_polynomial_multiplication() {
        // constant == 2
        let constant_polynomial = P17::constant(FieldElement::from_u64(19_u64), 2);

        //5x2 + 10x1x2
        let multilinear_polynomial = P17::new(vec![
            Monomial::new(FieldElement::from_u64(5), vec![0, 1]).unwrap(),
            Monomial::new(FieldElement::from_u64(10), vec![1, 1]).unwrap(),
        ])
        .unwrap();

        let result = multilinear_polynomial * constant_polynomial;

        let expected = P17::new(vec![
            Monomial::new(FieldElement::from_u64(10_u64), vec![0, 1]).unwrap(),
            Monomial::new(FieldElement::from_u64(20_u64), vec![1, 1]).unwrap(),
        ])
        .unwrap();

        assert_eq!(
            result.evaluate(&[FieldElement::zero(), FieldElement::one()]),
            expected.evaluate(&[FieldElement::zero(), FieldElement::one()]),
        );

        let lagrange_zero = P17::new(vec![
            Monomial::new(FieldElement::one(), vec![1, 0]).unwrap(),
        ])
        .unwrap();

        let lagrange_one = P17::new(vec![
            Monomial::new(FieldElement::one(), vec![0, 0]).unwrap(),
            Monomial::new(FieldElement::from_i64(-1), vec![0, 1]).unwrap(),
        ])
        .unwrap();

        let result = lagrange_one * lagrange_zero;

        let expected = P17::new(vec![
            Monomial::new(FieldElement::one(), vec![1, 0]).unwrap(),
            Monomial::new(FieldElement::from_i64(-1), vec![1, 1]).unwrap(),
        ])
        .unwrap();

        assert_eq!(
            result
                .evaluate(&[FieldElement::one(), FieldElement::zero()])
                .unwrap(),
            expected
                .evaluate(&[FieldElement::one(), FieldElement::zero()])
                .unwrap()
        );
    }
}
