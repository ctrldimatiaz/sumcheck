use crate::{
    field::field_element::FieldElement,
    polynomials::{monomial::Monomial, multilinear::MultilinearPolynomial, polynomial::Polynomial},
    protocol::{prover::Prover, sumcheck::SumCheck, verifier::Verifier},
};

pub struct ToyExample<const P: u64> {}

impl<const P: u64> ToyExample<P> {
    pub fn new() -> Self {
        ToyExample {}
    }

    pub fn run(&self) -> bool {
        //x2x3 + 2x1 + 5x1x2
        let polynomial: Polynomial<P> = Polynomial::new(vec![
            Monomial::new(FieldElement::from_u64(1), vec![0, 1, 1]).unwrap(),
            Monomial::new(FieldElement::from_u64(2), vec![1, 0, 0]).unwrap(),
            Monomial::new(FieldElement::from_u64(5), vec![1, 1, 0]).unwrap(),
        ])
        .unwrap();

        let multilinear = MultilinearPolynomial::new(polynomial).unwrap();

        let prover = Prover::new(multilinear);

        let claimed_sum = prover.claimed_sum();

        let verifier = Verifier::new(claimed_sum);

        let mut sumcheck = SumCheck::new(prover, verifier, 3);

        sumcheck.verify()
    }
}
