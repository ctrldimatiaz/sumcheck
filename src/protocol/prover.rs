use crate::{field::field_element::FieldElement, polynomial::multilinear::MultilinearPolynomial};

pub struct Prover<const P: u64> {
    polynomial: MultilinearPolynomial<P>,
}

impl<const P: u64> Prover<P> {
    pub fn new(polynomial: MultilinearPolynomial<P>) -> Self {
        Prover { polynomial }
    }

    pub fn compute_round(&self, _round: &Vec<FieldElement<P>>) -> FieldElement<P> {
        FieldElement::one()
    }
}
