#[derive(Debug, Copy, Clone, PartialEq)]
pub struct BooleanCircuit<const P: u64> {
    circuit: vec![ArithmeticOperation<P>],
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ArithmeticOperation<const P: u64> {
    // FieldElements consist of zero or one
    boolOperation: BoolOperation,
    leftValue: FieldElement<P>,
    rightValue: FieldElement<P>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum BoolOperation {
    AND(u64, u64),
    OR(u64, u64),
    NOT(usize),
}

impl BoolOperation {
    // Calculate value according to arithemtic formula
    pub fn calculate(&self) -> FieldElement<P> {
        match self {
            Self::AND(x1, x2) => x1 * x2,    // (x1 AND x2) = x1*x2
            Self::NOT(v) => 1 - v,           // NOT(x) = 1 - x
            Self::OR(l, r) => l + r - l * r, // (x1 OR x2) = x1 + x2 - x1 *x2
        }
    }
}

impl<const P: u64> ArithmeticOperation {
    pub fn new(
        boolOperation: BoolOperation,
        leftValue: FieldElement<P>,
        rightValue: FieldElement<P>,
    ) -> Result<Self, BoolOperationParseError> {
        if (leftValue != 0 && leftValue != 1) || (rightValue != 0 && rightValue != 1) {
            BoolOperationParseError::ValuesMustBeOneOrZero
        }

        Self {
            boolOperation,
            leftValue,
            rightValue,
        }
    }
}
