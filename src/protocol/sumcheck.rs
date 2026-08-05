use log::error;

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

        for i in 0..self.no_of_rounds {
            match self.prover.compute_round(&round) {
                Ok(gn) => match self.verifier.check_round(gn) {
                    Ok(rn) => {
                        round.push(rn);
                    }
                    Err(e) => {
                        error!("Error in verifier at round {} {}", i, e);
                        return false;
                    }
                },
                Err(e) => {
                    error!("{}", e);
                    return false;
                }
            }
        }

        true
    }
}
