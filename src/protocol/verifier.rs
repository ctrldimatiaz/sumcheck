use crate::field::field_element::FieldElement;

//verifier will check the claimed_sum on each round
pub struct Verifier<const P: u64> {
    claimed_sum: FieldElement<P>,
}

impl<const P: u64> Verifier<P> {}
