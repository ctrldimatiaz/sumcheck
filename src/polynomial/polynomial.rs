use crate::polynomial::monomial::Monomial;

pub struct Polynomial<const P: u64> {
    pub terms: Vec<Monomial<P>>,
}
