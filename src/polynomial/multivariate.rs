use crate::FieldElement;

pub struct MultivariatePolynomial<const P: u64> {
    variable_coefficients: Vec<Vec<FieldElement<P>>>,
}

impl<const P: u64> MultivariatePolynomial<P> {
    pub fn from_variables_vec(vector: Vec<Vec<FieldElement<P>>>) -> Self {
        Self {
            variable_coefficients: vector,
        }
    }

    pub fn evaluate(self, elements: &Vec<FieldElement<P>>) -> FieldElement<P> {
        let mut result: FieldElement<P> = FieldElement::zero();

        if elements.len() != self.variable_coefficients.len() {
            return result;
        }

        for (variable_index, variable_coefficients) in self.variable_coefficients.iter().enumerate()
        {
            for (index, item) in variable_coefficients.iter().enumerate() {
                result = result + (*item * elements[variable_index].pow(index as u64));
            }
        }

        result
    }
}
