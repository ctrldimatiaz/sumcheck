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
        polynomial_previous_gn: Option<Polynomial<P>>,
    ) -> Result<FieldElement<P>, ProtocolError> {
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

        let evaluated_polynomial =
            polynomial.evaluate(&values_zero).unwrap() + polynomial.evaluate(&values_one).unwrap();

        if round_rn.is_none() {
            if self.claimed_sum == evaluated_polynomial {
                return Ok(self.generate_rn());
            }

            return Err(ProtocolError::InvalidClaim);
        }

        let rn = round_rn.unwrap();

        let previous_gn = polynomial_previous_gn.unwrap();

        values_zero = vec![FieldElement::<P>::zero(); previous_gn.get_number_of_variables() - 1];
        values_zero.push(rn);

        let grn = previous_gn.evaluate(&values_zero).unwrap();

        if grn == evaluated_polynomial {
            return Ok(self.generate_rn());
        }

        Err(ProtocolError::InvalidClaim)
    }

    pub fn final_round(
        &self,
        last_gn: &Polynomial<P>,
        values: Vec<FieldElement<P>>,
    ) -> Result<FieldElement<P>, ProtocolError> {
        let result = FieldElement::one();

        let mut values_zero =
            vec![FieldElement::<P>::zero(); last_gn.get_number_of_variables() - 1];
        let last_rn = values.last().unwrap();

        values_zero.push(*last_rn);

        Ok(result)
    }

    fn generate_rn(&self) -> FieldElement<P> {
        let mut rng = rand::rng();

        let rng_field_value = rng.random_range(0..P);

        FieldElement::from_u64(rng_field_value)
    }
}
