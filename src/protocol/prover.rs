use crate::polynomial::multilinear::MultilinearPolynomial;

pub struct Prover<const P: u64> {
    polynomial: MultilinearPolynomial<P>,
}

impl<const P: u64> Prover<P> {
    pub fn new(polynomial: MultilinearPolynomial<P>) -> Self {
        Prover { polynomial }
    }
}
