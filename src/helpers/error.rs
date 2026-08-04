#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PolynomialError {
    NotMultilinear,
    DifferentVariableCounts,
    EmptyPolynomial,
    ConstantPolynomial,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ProtocolError {
    InvalidClaim,
}
