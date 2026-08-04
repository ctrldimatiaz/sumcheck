use crate::protocol::{prover::Prover, verifier::Verifier};

pub struct SumCheck<const P: u64> {
    prover: Prover<P>,
    verifier: Verifier<P>,
}

impl<const P: u64> SumCheck<P> {
    pub fn verify(mut self) -> bool {
        let h1 = self.prover.claimed_sum();

        self.verifier = Verifier::new(h1);

        let mut round = vec![];

        let g1 = self.prover.compute_round(&round).unwrap();

        let r1 = self.verifier.check_round(g1).unwrap();

        round.push(r1);

        println!("{}", r1);

        true
    }
}
