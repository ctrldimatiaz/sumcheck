use std::fmt::Display;

use itertools::Itertools;

use crate::FieldElement;

pub struct Monomial<const P: u64> {
    pub coefficient: FieldElement<P>,
    pub exponents: Vec<usize>,
}

impl<const P: u64> Monomial<P> {
    pub fn is_multilinear(&self) -> bool {
        self.exponents.iter().all(|&e| e <= 1)
    }

    pub fn evaluate(&self, values: &Vec<FieldElement<P>>) -> FieldElement<P> {
        let mut result: FieldElement<P> = self.coefficient;

        for (index, exponent) in self.exponents.iter().enumerate() {
            result = result * values[index].pow(*exponent as u64);
        }

        result
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
