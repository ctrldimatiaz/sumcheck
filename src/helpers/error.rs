use core::fmt;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PolynomialError {
    NotMultilinear,
    DifferentVariableCounts,
    EmptyPolynomial,
    ConstantPolynomial,
}

impl fmt::Display for PolynomialError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotMultilinear => write!(f, "Polynomial is not multilinear"),
            Self::DifferentVariableCounts => {
                write!(f, "Variable exponent or variable value is missing")
            }
            Self::EmptyPolynomial => write!(f, "Polynomial is empty"),
            Self::ConstantPolynomial => write!(f, "Polynomial is constant. No use for it."),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ProtocolError {
    InvalidClaim,
}

impl fmt::Display for ProtocolError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidClaim => write!(f, "Invalid Claim."),
        }
    }
}
