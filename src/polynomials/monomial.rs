use std::{fmt::Display, ops::Mul};

use itertools::Itertools;
use log::debug;

use crate::{FieldElement, helpers::error::PolynomialError};

#[derive(Debug, Clone, PartialEq)]
pub struct Monomial<const P: u64> {
    coefficient: FieldElement<P>,
    exponents: Vec<usize>,
}

impl<const P: u64> Monomial<P> {
    pub fn new(
        coefficient: FieldElement<P>,
        exponents: Vec<usize>,
    ) -> Result<Self, PolynomialError> {
        if exponents.is_empty() {
            return Err(PolynomialError::EmptyPolynomial);
        }

        Ok(Self {
            coefficient,
            exponents,
        })
    }

    pub fn constant(coefficient: FieldElement<P>, size: usize) -> Self {
        Self {
            coefficient,
            exponents: vec![0; size],
        }
    }

    pub fn exponents(&self) -> &[usize] {
        &self.exponents
    }

    pub fn coefficient(&self) -> FieldElement<P> {
        self.coefficient
    }

    //Check if monomial is is multilinear
    pub fn is_multilinear(&self) -> bool {
        self.exponents.iter().all(|&e| e <= 1) || self.coefficient == FieldElement::zero()
    }

    // Check if monomial is constant.
    pub fn is_constant(&self) -> bool {
        self.exponents.iter().all(|&e| e == 0) || self.coefficient == FieldElement::zero()
    }

    // Number of variables in polynomial
    pub fn get_number_of_variables(&self) -> usize {
        self.exponents.len()
    }

    // Evaluate the monomial according to the variables values passed
    pub fn evaluate(&self, values: &[FieldElement<P>]) -> Result<FieldElement<P>, PolynomialError> {
        if values.len() != self.exponents.len() {
            return Err(PolynomialError::DifferentVariableCounts);
        }

        let mut result: FieldElement<P> = self.coefficient;

        for (index, exponent) in self.exponents.iter().enumerate() {
            result = result * values[index].pow(*exponent as u64);
        }

        Ok(result)
    }

    // Reduce the monomial to the fixed_value variable index with fixed values in argument values.
    // Ex. 0 -> returns x1 monomial, fixed_value = 1 and values = 17 reduce to x2 with x1=17
    pub fn evaluate_with_fixing_term(
        &self,
        fixed_value: u64,
        values: &[FieldElement<P>],
    ) -> Monomial<P> {
        debug!(
            "Received values at monomial evaluation: {} and index {}",
            values.iter().join(" , "),
            fixed_value,
        );

        let mut coefficient: FieldElement<P> = self.coefficient;
        let mut result_exponent: usize = 0;

        for (index, exponent) in self.exponents.iter().enumerate() {
            if fixed_value == index as u64 {
                if *exponent == 1 {
                    result_exponent = 1;
                }
            } else {
                //we will have minus 1 variable in values

                let values_index = if index > 0 && fixed_value < (self.exponents.len() - 1) as u64 {
                    index - 1
                } else {
                    index
                };

                coefficient = coefficient * values[values_index].pow(*exponent as u64);
            }
        }

        let mut exponents = vec![0; fixed_value as usize];

        exponents.push(result_exponent);

        debug!(
            "Created monomial with exponents: {}",
            exponents.iter().join(" , ")
        );

        Monomial {
            coefficient,
            exponents,
        }
    }
}

// Display
impl<const P: u64> Display for Monomial<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            self.coefficient,
            self.exponents
                .iter()
                .enumerate()
                .map(|(index, &exp)| {
                    if exp == 0 {
                        String::new()
                    } else if exp == 1 {
                        format!("x{}", index + 1)
                    } else {
                        format!("x{}^{}", index + 1, exp)
                    }
                })
                .join("")
        )
    }
}

impl<const P: u64> Mul for Monomial<P> {
    type Output = Self;
    fn mul(self, rhs: Monomial<P>) -> Monomial<P> {
        let coefficient = self.coefficient * rhs.coefficient;
        let exponents = self
            .exponents
            .iter()
            .enumerate()
            .map(|(index, e)| e + rhs.exponents[index])
            .collect();

        Self::new(coefficient, exponents).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type M17 = Monomial<17>;
    type F17 = FieldElement<17>;

    #[test]
    fn test_zero_evaluation() {
        let monomial = M17::new(FieldElement::one(), vec![1, 2]).unwrap();
        assert_eq!(
            monomial.evaluate(&vec![F17::zero(); 2]).unwrap(),
            FieldElement::zero()
        );
    }

    #[test]
    fn test_monomial_multilinearity() {
        let coeff = FieldElement::from_u64(5u64);

        let monomial = M17::new(coeff, vec![1, 1]).unwrap();
        let monomial_two = M17::new(coeff, vec![0, 1]).unwrap();
        let monomial_three = M17::new(coeff, vec![1, 0]).unwrap();
        let monomial_four = M17::new(coeff, vec![0, 0]).unwrap();

        let monomial_not_multilinear = M17::new(coeff, vec![4, 0]).unwrap();
        let monomial_linear_with_coeff_zero = M17::new(FieldElement::zero(), vec![4, 0]).unwrap();

        assert!(monomial.is_multilinear());
        assert!(monomial_two.is_multilinear());
        assert!(monomial_three.is_multilinear());
        assert!(monomial_four.is_multilinear());
        assert!(!monomial_not_multilinear.is_multilinear());
        assert!(monomial_linear_with_coeff_zero.is_constant());
    }

    #[test]
    fn test_monomial_constantness() {
        let coeff = FieldElement::from_u64(5u64);
        let monomial = M17::new(coeff, vec![1, 1]).unwrap();
        let monomial_two = M17::new(coeff, vec![0, 1]).unwrap();
        let monomial_three = M17::new(coeff, vec![1, 0]).unwrap();
        let monomial_four = M17::new(coeff, vec![0, 0]).unwrap();

        let monomial_not_multilinear = M17::new(FieldElement::zero(), vec![4, 0]).unwrap();

        assert!(!monomial.is_constant());
        assert!(!monomial_two.is_constant());
        assert!(!monomial_three.is_constant());
        assert!(monomial_four.is_constant());
        assert!(monomial_not_multilinear.is_constant());
    }

    #[test]
    fn test_monomial_terms() {
        let coeff = FieldElement::from_u64(5u64);
        let monomial = M17::new(coeff, vec![1, 1]).unwrap();

        assert_eq!(monomial.get_number_of_variables(), 2);
    }

    #[test]
    fn test_monomial_multiplication() {
        let coeff = FieldElement::from_u64(2u64);
        let monomial_one = M17::new(coeff, vec![0, 0]).unwrap();
        let coeff = FieldElement::from_u64(1u64);
        let monomial = M17::new(coeff, vec![1, 1]).unwrap();

        let result = monomial_one * monomial;

        assert_eq!(
            result,
            Monomial::new(FieldElement::from_u64(2_u64), vec![1, 1]).unwrap()
        );
    }
}
