use log::{error, info};

use crate::{
    field::field_element::FieldElement,
    polynomial::polynomial::Polynomial,
    protocol::{prover::Prover, verifier::Verifier},
};

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
        let mut round_rn: Option<FieldElement<P>> = None;
        let mut round_gn: Vec<Polynomial<P>> = vec![];
        let mut previous_gn: Option<Polynomial<P>> = None;

        for i in 0..self.no_of_rounds {
            //prover returns gn univariate polynomial
            match self.prover.compute_round(&round) {
                Ok(gn) => {
                    info!(
                        "Prover computed round {} with gn {} with number of terms {}",
                        i,
                        gn,
                        gn.get_number_of_terms()
                    );

                    if round.len() > 0 {
                        round_rn = Some(*round.last().unwrap());
                        previous_gn = Some(round_gn[(i - 1) as usize].clone());
                    }

                    round_gn.push(gn);

                    //if round is ok verifier return the rn
                    match self.verifier.check_round(
                        &round_gn[i as usize],
                        round_rn,
                        previous_gn.clone(),
                    ) {
                        Ok(rn) => {
                            info!(
                                "Verifier computed round {} with gn {} and rn {}",
                                i, round_gn[i as usize], rn
                            );
                            round.push(rn);
                        }
                        Err(e) => {
                            error!("Error in verifier at round {} | {}", i, e);
                            return false;
                        }
                    }
                }
                Err(e) => {
                    error!("Error in prover at round {} | {}", i, e);
                    return false;
                }
            }
        }

        //final round
        //there should be the oracle call and match with gn(rn) = g(r1,r2...rn)

        let v = round_gn.last().unwrap();
        match self.verifier.final_round(v, round) {
            Ok(_rn) => {
                info!("Verifier computed final round successfully.");
            }
            Err(e) => {
                error!("Error in verifier at last round: {}", e);
                return false;
            }
        }

        true
    }
}
