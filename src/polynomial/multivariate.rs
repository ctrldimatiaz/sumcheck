use crate::FieldElement;

pub struct multivariate_polynomial<const P: u64> {
    variable_coefficients: Vec<Vec<FieldElement<P>>>,
}
