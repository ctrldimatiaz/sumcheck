use rand::RngExt;

use crate::{
    field::field_element::FieldElement, helpers::error::ProtocolError,
    polynomial::polynomial::Polynomial,
};

//verifier will check the claimed_sum on each round
pub struct Verifier<const P: u64> {
    claimed_sum: FieldElement<P>,
}

impl<const P: u64> Verifier<P> {
    pub fn new(claimed_sum: FieldElement<P>) -> Self {
        Verifier { claimed_sum }
    }

    pub fn check_round(&self, polynomial: Polynomial<P>) -> Result<FieldElement<P>, ProtocolError> {
        let evaluated_polynomial = polynomial.evaluate(&vec![FieldElement::zero()]).unwrap()
            + polynomial.evaluate(&vec![FieldElement::one()]).unwrap();

        if self.claimed_sum == evaluated_polynomial {
            return Ok(self.generate_rn());
        }
        Err(ProtocolError::InvalidClaim)
    }

    fn generate_rn(&self) -> FieldElement<P> {
        let mut rng = rand::rng();

        let rng_field_value = rng.random_range(0..P);

        FieldElement::from_u64(rng_field_value)
    }
}
