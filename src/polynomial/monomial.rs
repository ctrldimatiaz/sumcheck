use std::fmt::Display;

use itertools::Itertools;

use crate::FieldElement;

pub struct Monomial<const P: u64> {
    coefficient: FieldElement<P>,
    exponents: Vec<usize>,
}

impl<const P: u64> Monomial<P> {
    pub fn is_multilinear(&self) -> bool {
        self.exponents.iter().all(|&e| e <= 1)
    }

    pub fn is_constant(&self) -> bool {
        self.exponents.iter().all(|&e| e == 0)
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

    pub fn new(coefficient: FieldElement<P>, exponents: Vec<usize>) -> Self {
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
                        format!("x{}", index)
                    } else {
                        format!("x{}^{}", index, exp)
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
}
