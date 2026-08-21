use std::fmt::Display;

use crate::{
    field::field_element::FieldElement,
    helpers::{error::PolynomialError, functions::number_to_bits_vec},
    polynomials::{monomial::Monomial, polynomial::Polynomial},
};

#[derive(Debug, Clone, PartialEq)]
pub struct MultilinearPolynomial<const P: u64> {
    polynomial: Polynomial<P>,
}

impl<const P: u64> MultilinearPolynomial<P> {
    pub fn new(polynomial: Polynomial<P>) -> Result<Self, PolynomialError> {
        if !polynomial.is_multilinear() {
            return Err(PolynomialError::NotMultilinear);
        }

        if polynomial.is_constant() {
            return Err(PolynomialError::ConstantPolynomial);
        }

        Ok(Self { polynomial })
    }

    #[allow(dead_code)]
    pub fn evaluate(
        &self,
        values: Vec<FieldElement<P>>,
    ) -> Result<FieldElement<P>, PolynomialError> {
        self.polynomial.evaluate(&values)
    }

    // Constructing the univariate polynomial gj(Xj)
    // Ex.: [] -> return polynomial with X1 fixed,
    // [r1] would return polynomial with X2 fixed and x1 = r1
    pub fn reduce_polynomial(
        &self,
        round: &[FieldElement<P>],
    ) -> Result<Polynomial<P>, PolynomialError> {
        self.polynomial.reduce_polynomial(round)
    }

    // Computing sum of the polynomial
    pub fn compute_sum(&self) -> FieldElement<P> {
        self.polynomial.compute_sum()
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
            if *value == FieldElement::<P>::zero() {
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
        let size = self.polynomial.get_number_of_variables();
        let combinations = 2_u64.pow(size as u32);
        let f_tilde: Polynomial<P>;

        // evaluate through all the combinations according to the number of variables
        // ex.: 3 variables would go through 0 - (0,0,0), 1 - (0,0,1)... and would have 2³ = 8 combinations
        for i in 0..combinations {
            let values: Vec<FieldElement<P>> = number_to_bits_vec(i, size)
                .iter()
                .map(|bit| FieldElement::from_u64(*bit))
                .collect();

            let evaluation_result = self.polynomial.evaluate(&values).unwrap();

            let langrage_polynomial =
                Self::langrage_basis(&values)? * Polynomial::constant(evaluation_result, size);
        }

        Ok(self.clone())
    }
}

// Display
impl<const P: u64> Display for MultilinearPolynomial<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.polynomial.get_readable_polynomial())
    }
}

#[cfg(test)]
mod tests {
    use crate::polynomials::monomial::Monomial;

    use super::*;

    type M17 = MultilinearPolynomial<17>;
    type P17 = Polynomial<17>;

    #[test]
    fn test_multilinear_polynomial_constantness() {
        let polynomial = P17::new(vec![
            Monomial::new(FieldElement::from_u64(5), vec![0, 0]).unwrap(),
            Monomial::new(FieldElement::from_u64(5), vec![0, 0]).unwrap(),
        ])
        .unwrap();

        let multilinear = M17::new(polynomial).unwrap_err();

        assert_eq!(multilinear, PolynomialError::ConstantPolynomial);
    }

    #[test]
    fn test_multilinear_polynomial_multilinearity() {
        let poly = P17::new(vec![
            Monomial::new(FieldElement::from_u64(5), vec![0, 0]).unwrap(),
            Monomial::new(FieldElement::from_u64(5), vec![2, 2]).unwrap(),
        ])
        .unwrap();

        let multilinear = M17::new(poly).unwrap_err();

        assert_eq!(multilinear, PolynomialError::NotMultilinear);
    }

    #[test]
    fn test_multilinear_polynomial_evaluation() {
        let poly = P17::new(vec![
            Monomial::new(FieldElement::from_u64(5), vec![0, 1]).unwrap(),
            Monomial::new(FieldElement::from_u64(5), vec![1, 1]).unwrap(),
        ])
        .unwrap();

        let values = vec![FieldElement::from_u64(0), FieldElement::from_u64(5)];

        let multilinear = M17::new(poly).unwrap();

        let result = multilinear.evaluate(values).unwrap();

        assert_eq!(result, FieldElement::from_u64(8));
    }
}
