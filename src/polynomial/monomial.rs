use std::fmt::Display;

use itertools::Itertools;

use crate::FieldElement;

#[derive(Debug, Clone, PartialEq)]
pub struct Monomial<const P: u64> {
    coefficient: FieldElement<P>,
    exponents: Vec<usize>,
}

impl<const P: u64> Monomial<P> {
    pub fn new(coefficient: FieldElement<P>, exponents: Vec<usize>) -> Self {
        Monomial {
            coefficient,
            exponents,
        }
    }

    pub fn is_multilinear(&self) -> bool {
        self.exponents.iter().all(|&e| e <= 1) || self.coefficient == FieldElement::zero()
    }

    pub fn is_constant(&self) -> bool {
        self.exponents.iter().all(|&e| e == 0) || self.coefficient == FieldElement::zero()
    }

    pub fn get_number_of_terms(&self) -> usize {
        self.exponents.len()
    }

    pub fn evaluate(&self, values: &Vec<FieldElement<P>>) -> FieldElement<P> {
        let mut result: FieldElement<P> = self.coefficient;

        for (index, exponent) in self.exponents.iter().enumerate() {
            result = result * values[index].pow(*exponent as u64);
        }

        result
    }

    pub fn evaluate_with_fixing_term(
        &self,
        fixed_value: u64,
        values: &Vec<FieldElement<P>>,
    ) -> Monomial<P> {
        let mut coefficient: FieldElement<P> = self.coefficient;
        let mut result_exponent: usize = 0;

        for (index, exponent) in self.exponents.iter().enumerate() {
            if fixed_value == index as u64 {
                if *exponent == 1 {
                    result_exponent = 1;
                }
            } else {
                //we will have minus 1 variable in values
                coefficient = coefficient * values[index - 1].pow(*exponent as u64);
            }
        }

        let exponents = vec![result_exponent];

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

#[cfg(test)]
mod tests {
    use super::*;

    type M17 = Monomial<17>;
    type F17 = FieldElement<17>;

    #[test]
    fn test_zero_evaluation() {
        let monomial = M17::new(FieldElement::zero(), vec![1, 2]);
        assert_eq!(
            monomial.evaluate(&vec![F17::zero(), F17::zero()]),
            FieldElement::zero()
        );
    }

    #[test]
    fn test_monomial_multilinearity() {
        let coeff = FieldElement::from_u64(5u64);

        let monomial = M17::new(coeff, vec![1, 1]);
        let monomial_two = M17::new(coeff, vec![0, 1]);
        let monomial_three = M17::new(coeff, vec![1, 0]);
        let monomial_four = M17::new(coeff, vec![0, 0]);

        let monomial_not_multilinear = M17::new(coeff, vec![4, 0]);
        let monomial_linear_with_coeff_zero = M17::new(FieldElement::zero(), vec![4, 0]);

        assert!(monomial.is_multilinear());
        assert!(monomial_two.is_multilinear());
        assert!(monomial_three.is_multilinear());
        assert!(monomial_four.is_multilinear());
        assert!(monomial_linear_with_coeff_zero.is_multilinear());
        assert!(!monomial_not_multilinear.is_multilinear());
    }

    #[test]
    fn test_monomial_constantness() {
        let coeff = FieldElement::from_u64(5u64);
        let monomial = M17::new(coeff, vec![1, 1]);
        let monomial_two = M17::new(coeff, vec![0, 1]);
        let monomial_three = M17::new(coeff, vec![1, 0]);
        let monomial_four = M17::new(coeff, vec![0, 0]);

        let monomial_not_multilinear = M17::new(FieldElement::zero(), vec![4, 0]);

        assert!(!monomial.is_constant());
        assert!(!monomial_two.is_constant());
        assert!(!monomial_three.is_constant());
        assert!(monomial_four.is_constant());
        assert!(monomial_not_multilinear.is_constant());
    }

    #[test]
    fn test_monomial_terms() {
        let coeff = FieldElement::from_u64(5u64);
        let monomial = M17::new(coeff, vec![1, 1]);

        assert_eq!(monomial.get_number_of_terms(), 2);
    }
}
