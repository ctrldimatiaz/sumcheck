use crate::{
    field::field_element::FieldElement,
    polynomials::{monomial::Monomial, multilinear::MultilinearPolynomial, polynomial::Polynomial},
    protocol::{oracle::Oracle, prover::Prover, sumcheck::SumCheck, verifier::Verifier},
};

pub struct BoolCircuitExamples<const P: u64> {}

impl<const P: u64> BoolCircuitExamples<P> {
    pub fn new() -> Self {
        BoolCircuitExamples {}
    }

    pub fn run(&self) -> bool {}
}
