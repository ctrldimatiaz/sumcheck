use log::{error, info};

use crate::polynomials::polynomial::Polynomial;

pub struct MultilinearExtensionExample<'a, const P: u64> {
    polynomial: &'a Polynomial<P>,
}

impl<'a, const P: u64> MultilinearExtensionExample<'a, P> {
    pub fn new(polynomial: &'a Polynomial<P>) -> Self {
        Self { polynomial }
    }

    pub fn generate_f_tilde(&self) -> bool {
        match self.polynomial.generate_f_tilde() {
            Ok(poly) => {
                info!("Succesfully generated f tilde: {}", poly);
                true
            }
            Err(e) => {
                error!("Error generating f tilde at mleexample: {}", e);
                false
            }
        }
    }
}
