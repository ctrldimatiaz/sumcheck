use crate::protocol::{prover::Prover, verifier::Verifier};

pub struct SumCheck<const P: u64> {
    prover: Prover<P>,
    verifier: Verifier<P>,
}

impl<const P: u64> SumCheck<P> {
    pub fn verify(mut self) -> bool {
        let h1 = self.prover.claimed_sum();

        self.verifier = Verifier::new(h1);

        true
    }
}
