use crate::{
    field::field_element::FieldElement,
    polynomials::{monomial::Monomial, multilinear::MultilinearPolynomial, polynomial::Polynomial},
    protocol::{oracle::Oracle, prover::Prover, sumcheck::SumCheck, verifier::Verifier},
};

pub struct DishonestProver<const P: u64> {}

impl<const P: u64> DishonestProver<P> {
    pub fn new() -> Self {
        DishonestProver {}
    }

    // Function that runs the full interaction based on a dishonest prover
    pub fn run(&self) -> bool {
        //2x3 + 3x1x2 + 7x1x3
        let dishonest_polynomial: Polynomial<P> = Polynomial::new(vec![
            Monomial::new(FieldElement::from_u64(2), vec![0, 0, 1]).unwrap(),
            Monomial::new(FieldElement::from_u64(3), vec![1, 1, 0]).unwrap(),
            Monomial::new(FieldElement::from_u64(7), vec![1, 0, 1]).unwrap(),
        ])
        .unwrap();

        //x2x3 + 2x1 + 5x1x2
        let polynomial: Polynomial<P> = Polynomial::new(vec![
            Monomial::new(FieldElement::from_u64(1), vec![0, 1, 1]).unwrap(),
            Monomial::new(FieldElement::from_u64(2), vec![1, 0, 0]).unwrap(),
            Monomial::new(FieldElement::from_u64(5), vec![1, 1, 0]).unwrap(),
        ])
        .unwrap();

        // Initializations of needed structs to run the sumcheck interaction
        let multilinear = MultilinearPolynomial::new(polynomial).unwrap();

        let dishonest_multilinear = MultilinearPolynomial::new(dishonest_polynomial).unwrap();

        let prover = Prover::new(dishonest_multilinear.clone());

        let oracle = Oracle::new(multilinear);

        let claimed_sum = prover.claimed_sum();

        let verifier = Verifier::new(claimed_sum, oracle);

        // Run sumcheck
        let mut sumcheck = SumCheck::new(prover, verifier, 3);

        sumcheck.verify()
    }
}
