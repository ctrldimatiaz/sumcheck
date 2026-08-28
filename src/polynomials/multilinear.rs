use std::fmt::Display;

use crate::{
    field::field_element::FieldElement, helpers::error::PolynomialError,
    polynomials::polynomial::Polynomial,
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
    pub fn evaluate(&self, values: &[FieldElement<P>]) -> Result<FieldElement<P>, PolynomialError> {
        self.polynomial.evaluate(values)
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
