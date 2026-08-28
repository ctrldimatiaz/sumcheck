use crate::{
    field::field_element::FieldElement,
    helpers::{error::PolynomialError, functions::number_to_bits_vec, mleinput::MleInput},
    polynomials::{multilinear::MultilinearPolynomial, polynomial::Polynomial},
};

pub struct MultilinearExtensionFromEvaluations<const P: u64> {
    input: MleInput<P>,
}

impl<'a, const P: u64> MultilinearExtensionFromEvaluations<P> {
    pub fn new(input: MleInput<P>) -> Self {
        Self { input }
    }

    // Here we generate f tilde according to a given polynomial
    pub fn generate_f_tilde(&self) -> Result<MultilinearPolynomial<P>, PolynomialError> {
        let size = self.input.vector.len();
        let combinations = self.input.evaluations.len();
        let mut f_tilde: Polynomial<P> = Polynomial::constant(FieldElement::zero(), size);

        // evaluate through all the combinations according to the number of variables
        // ex.: 3 variables would go through 0 - (0,0,0), 1 - (0,0,1)... and would have 2³ = 8 combinations
        for i in 0..combinations {
            let values: Vec<FieldElement<P>> = number_to_bits_vec(i as u64, size)
                .iter()
                .map(|bit| FieldElement::from_u64(*bit))
                .collect();

            let evaluation_result = self.input.evaluations[i];

            //get langrage polynomial
            let langrage_polynomial = Polynomial::langrage_basis(&values)?
                * Polynomial::constant(evaluation_result, size);

            f_tilde = f_tilde + langrage_polynomial;
        }

        let result = MultilinearPolynomial::new(f_tilde)?;

        Ok(result)
    }

    pub fn get_values(&self) -> &[FieldElement<P>] {
        &self.input.vector
    }
}
