use crate::{
    field::field_element::FieldElement,
    helpers::error::PolynomialError,
    polynomials::{multilinear::MultilinearPolynomial, polynomial::Polynomial},
};

pub struct Prover<const P: u64> {
    polynomial: MultilinearPolynomial<P>,
}

impl<const P: u64> Prover<P> {
    pub fn new(polynomial: MultilinearPolynomial<P>) -> Self {
        Prover { polynomial }
    }

    pub fn claimed_sum(&self) -> FieldElement<P> {
        self.polynomial.compute_sum()
    }

    //compute gn polynomial per each round
    pub fn compute_round(
        &self,
        round: &[FieldElement<P>],
    ) -> Result<Polynomial<P>, PolynomialError> {
        self.polynomial.reduce_polynomial(round)
    }
}
