use crate::{error::PolynomialError, polynomial::polynomial::Polynomial};

pub struct MultilinearPolynomial<const P: u64> {
    polynomial: Polynomial<P>,
}

impl<const P: u64> MultilinearPolynomial<P> {
    pub fn new(polynomial: Polynomial<P>) -> Result<Self, PolynomialError> {
        for term in &polynomial.terms {
            if !term.is_multilinear() {
                return Err(PolynomialError::NotMultilinear);
            }
        }

        Ok(Self { polynomial })
    }
}
