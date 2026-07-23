use crate::polynomial::monomial::Monomial;

pub struct Polynomial<const P: u64> {
    terms: Vec<Monomial<P>>,
}
