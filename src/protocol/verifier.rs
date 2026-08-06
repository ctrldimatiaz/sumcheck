use itertools::Itertools;
use log::debug;
use rand::RngExt;

use crate::{
    field::field_element::FieldElement, helpers::error::ProtocolError,
    polynomial::polynomial::Polynomial,
};

//the verifier will only check the claimed_sum on round one
pub struct Verifier<const P: u64> {
    claimed_sum: FieldElement<P>,
}

impl<const P: u64> Verifier<P> {
    pub fn new(claimed_sum: FieldElement<P>) -> Self {
        Verifier { claimed_sum }
    }

    pub fn check_round(
        &self,
        polynomial: &Polynomial<P>,
        round_rn: Option<FieldElement<P>>,
        polynomial_previousgn: Option<Polynomial<P>>,
    ) -> Result<FieldElement<P>, ProtocolError> {
        debug!(
            "Polynomial at verifier {} with terms {}",
            polynomial,
            polynomial.get_number_of_terms()
        );

        let mut values_zero = vec![FieldElement::<P>::zero(); polynomial.get_number_of_terms() - 1];
        let mut values_one = vec![FieldElement::<P>::zero(); polynomial.get_number_of_terms() - 1];

        values_zero.push(FieldElement::zero());
        values_one.push(FieldElement::one());

        let evaluated_polynomial =
            polynomial.evaluate(&values_zero).unwrap() + polynomial.evaluate(&values_one).unwrap();

        match round_rn {
            Some(rn) => {
                let previous_gn = polynomial_previousgn.unwrap();

                values_zero =
                    vec![FieldElement::<P>::zero(); previous_gn.get_number_of_terms() - 1];
                values_zero.push(rn);

                let grn = previous_gn.evaluate(&values_zero).unwrap();

                if grn == evaluated_polynomial {
                    return Ok(self.generate_rn());
                }
            }
            None => {
                if self.claimed_sum == evaluated_polynomial {
                    return Ok(self.generate_rn());
                }
            }
        }

        Err(ProtocolError::InvalidClaim)
    }

    fn generate_rn(&self) -> FieldElement<P> {
        let mut rng = rand::rng();

        let rng_field_value = rng.random_range(0..P);

        FieldElement::from_u64(rng_field_value)
    }
}
