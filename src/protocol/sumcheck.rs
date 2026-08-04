use crate::protocol::{prover::Prover, verifier::Verifier};

pub struct SumCheck<const P: u64> {
    prover: Prover<P>,
    verifier: Verifier<P>,
    no_of_rounds: u64,
}

impl<const P: u64> SumCheck<P> {
    pub fn new(prover: Prover<P>, verifier: Verifier<P>, no_of_rounds: u64) -> Self {
        SumCheck {
            prover,
            verifier,
            no_of_rounds,
        }
    }

    pub fn verify(&mut self) -> bool {
        let mut round = vec![];

        //for _i in 0..self.no_of_rounds {
        let gn = self.prover.compute_round(&round).unwrap();

        let rn = self.verifier.check_round(gn).unwrap();

        println!("{}", rn);

        round.push(rn);
        //}

        true
    }
}
