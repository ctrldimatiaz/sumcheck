use crate::FieldElement;

pub struct MultivariatePolynomial<const P: u64> {
    variable_coefficients: Vec<Vec<FieldElement<P>>>,
}

impl<const P: u64> MultivariatePolynomial<P> {
    pub fn evaluate(self, element: Vec<FieldElement<P>>) -> FieldElement<P> {
        let mut result: FieldElement<P> = FieldElement::zero();

        for (variable_index, variable_coefficients) in self.variable_coefficients.iter().enumerate()
        {
            for (index, item) in variable_coefficients.iter().enumerate() {
                result = result + (*item * element[variable_index].pow(index as u64));
            }
        }

        result
    }
}
