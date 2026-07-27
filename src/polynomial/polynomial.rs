use crate::{
    error::PolynomialError, field::field_element::FieldElement, polynomial::monomial::Monomial,
};

pub struct Polynomial<const P: u64> {
    pub terms: Vec<Monomial<P>>,
}

impl<const P: u64> Polynomial<P> {
    pub fn new(values: Vec<Monomial<P>>) -> Result<Self, PolynomialError> {
        if values.is_empty() {
            return Err(PolynomialError::EmptyPolynomial);
        }
        Ok(Self { terms: values })
    }

    pub fn evaluate(
        &self,
        values: Vec<FieldElement<P>>,
    ) -> Result<FieldElement<P>, PolynomialError> {
        if self.terms.iter().any(|t| t.exponents.len() != values.len()) {
            return Err(PolynomialError::DifferentVariableCounts);
        }

        let mut result: FieldElement<P> = FieldElement::zero();

        for term in &self.terms {
            result = result + term.evaluate(&values);
        }

        Ok(result)
    }
}
