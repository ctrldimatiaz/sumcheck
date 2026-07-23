use crate::polynomial::polynomial::Polynomial;

pub struct MultilinearPolynomial<const P: u64> {
    polynomial: Polynomial<P>,
}
