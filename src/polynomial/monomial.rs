use crate::FieldElement;

pub struct Monomial<const P: u64> {
    pub coefficient: FieldElement<P>,
    pub exponents: Vec<usize>,
}

impl<const P: u64> Monomial<P> {
    pub fn is_multilinear(&self) -> bool {
        self.exponents.iter().all(|&e| e <= 1)
    }
}
