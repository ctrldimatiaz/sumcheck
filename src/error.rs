#[derive(Debug)]
pub enum PolynomialError {
    NotMultilinear,
    DifferentVariableCounts,
    EmptyPolynomial,
    ConstantPolynomial,
}
