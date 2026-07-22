use crate::FieldElement;

//we will store the coefficients an such that as a0 + a1*x + ... an*x^n,
pub struct Polynomial<const P: u64> {
    coefficients: Vec<FieldElement<P>>,
}
