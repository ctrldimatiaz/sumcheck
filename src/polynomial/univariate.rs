use crate::FieldElement;

//we will store the coefficients an such that as a0 + a1*x + ... an*x^n,
pub struct UnivariatePolynomial<const P: u64> {
    coefficients: Vec<FieldElement<P>>,
}

impl<const P: u64> UnivariatePolynomial<P> {
    pub fn from_field_vec(vector: Vec<FieldElement<P>>) -> Self {
        Self {
            coefficients: vector,
        }
    }

    pub fn evaluate(&self, element: &FieldElement<P>) -> FieldElement<P> {
        let mut result: FieldElement<P> = FieldElement::zero();

        for coeff in self.coefficients.iter().rev() {
            result = result * *element + *coeff;
        }

        result
    }
}
