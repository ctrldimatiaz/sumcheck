use core::fmt;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PolynomialError {
    NotMultilinear,
    DifferentVariableCounts,
    EmptyPolynomial,
    ConstantPolynomial,
    EmptyMonomial,
}

impl fmt::Display for PolynomialError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotMultilinear => write!(f, "Polynomial is not multilinear"),
            Self::DifferentVariableCounts => {
                write!(f, "Values provided differ from variables number.")
            }
            Self::EmptyPolynomial => write!(f, "Polynomial is empty"),
            Self::ConstantPolynomial => write!(f, "Polynomial is constant. No use for it."),
            Self::EmptyMonomial => write!(f, "Monomial is empty."),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ProtocolResponse {
    InvalidClaim,
    ValidClaim,
}

impl fmt::Display for ProtocolResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidClaim => write!(f, "Invalid Claim."),
            Self::ValidClaim => write!(f, "Invalid Claim."),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum InputParserError {
    ErrorParsingFieldElement,
    LengthMismatch,
}

impl fmt::Display for InputParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ErrorParsingFieldElement => write!(f, "Error parsing field element."),
            Self::LengthMismatch => write!(f, "Evaluations and input vector lengths mismatch."),
        }
    }
}
