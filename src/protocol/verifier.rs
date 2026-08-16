use log::debug;
use rand::RngExt;

use crate::{
    field::field_element::FieldElement, helpers::error::ProtocolResponse,
    polynomials::polynomial::Polynomial,
};

//the verifier will only check the claimed_sum on round one
pub struct Verifier<const P: u64> {
    claimed_sum: FieldElement<P>,
}

impl<const P: u64> Verifier<P> {
    pub fn new(claimed_sum: FieldElement<P>) -> Self {
        Verifier { claimed_sum }
    }

    // Check each round prover claim integrity and return rn where n = number of the round or
    // invalid claim error
    pub fn check_round(
        &self,
        polynomial: &Polynomial<P>,
        round_rn: Option<FieldElement<P>>,
        polynomial_previous_gn: Option<Polynomial<P>>,
    ) -> Result<FieldElement<P>, ProtocolResponse> {
        debug!(
            "Polynomial at verifier {} with terms {}",
            polynomial,
            polynomial.get_number_of_variables()
        );

        let mut values_zero =
            vec![FieldElement::<P>::zero(); polynomial.get_number_of_variables() - 1];
        let mut values_one =
            vec![FieldElement::<P>::zero(); polynomial.get_number_of_variables() - 1];

        values_zero.push(FieldElement::zero());
        values_one.push(FieldElement::one());

        let evaluated_polynomial = polynomial.evaluate(&values_zero).map_err(|e| {
            debug!(
                "There was an issue at the verifier obtaining the previous polynomial gn. {}",
                e
            );
            ProtocolResponse::InvalidClaim
        })? + polynomial.evaluate(&values_one).map_err(|e| {
            debug!(
                "There was an issue at the verifier obtaining the previous polynomial gn. {}",
                e
            );
            ProtocolResponse::InvalidClaim
        })?;

        // first round
        if round_rn.is_none() {
            if self.claimed_sum == evaluated_polynomial {
                return Ok(self.generate_rn());
            }

            return Err(ProtocolResponse::InvalidClaim);
        }

        // rounds 1...n
        let rn = round_rn.unwrap();

        let previous_gn = polynomial_previous_gn.ok_or_else(|| {
            debug!("There was an issue at the verifier obtaining the previous polynomial gn.");
            ProtocolResponse::InvalidClaim
        })?;

        values_zero = vec![FieldElement::<P>::zero(); previous_gn.get_number_of_variables() - 1];
        values_zero.push(rn);

        let grn = previous_gn.evaluate(&values_zero).map_err(|e| {
            debug!(
                "There was an issue at the verifier evaluating the previous polynomial gn. {}",
                e
            );
            ProtocolResponse::InvalidClaim
        })?;

        if grn == evaluated_polynomial {
            return Ok(self.generate_rn());
        }

        Err(ProtocolResponse::InvalidClaim)
    }

    // Check the evaluated polynomial with the value of gn(Xn) = g(x1, ..., xn)
    pub fn final_round(
        &self,
        last_gn: &Polynomial<P>,
        values: Vec<FieldElement<P>>,
    ) -> ProtocolResponse {
        let result = ProtocolResponse::ValidClaim;

        let mut values_zero =
            vec![FieldElement::<P>::zero(); last_gn.get_number_of_variables() - 1];

        match values.last() {
            Some(last_rn) => {
                values_zero.push(*last_rn);
            }
            None => {
                debug!("Error obtaining last rn at verifier.");
                return ProtocolResponse::InvalidClaim;
            }
        }

        result
    }

    // Generate rn value after computing gn successfully in each round.
    fn generate_rn(&self) -> FieldElement<P> {
        let mut rng = rand::rng();

        let rng_field_value = rng.random_range(0..P);

        FieldElement::from_u64(rng_field_value)
    }
}
