use crate::FieldElement;

//we will store the coefficients an such that as a0 + a1*x + ... an*x^n,
pub struct Polynomial<const P: u64> {
    coefficients: Vec<FieldElement<P>>,
}

impl<const P: u64> Polynomial<P> {
    pub fn from_field_vec(vector: Vec<FieldElement<P>>) -> Self {
        Self {
            coefficients: vector,
        }
    }

    pub fn evaluate(self, element: FieldElement<P>) -> FieldElement<P> {
        let mut result: FieldElement<P> = FieldElement::zero();

        for (index, item) in self.coefficients.iter().enumerate() {
            result = result + (*item * element.pow(index as u64));
            println!("Result is  {}", result);
        }

        result
    }
}
