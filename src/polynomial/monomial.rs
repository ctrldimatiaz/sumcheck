use crate::FieldElement;

pub struct Monomial<const P: u64> {
    coefficient: FieldElement<P>,
    exponents: Vec<usize>,
}
