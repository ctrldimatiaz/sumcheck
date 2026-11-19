#[derive(Debug, Copy, Clone, PartialEq)]
pub struct BooleanCircuit<const P: u64> {
    circuit: vec![ArithmeticOperation<P>],
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ArithmeticOperation<const P: u64> {
    // FieldElements consist of zero or one
    BoolOperation: BoolOperation,
    LeftValue: FieldElement<P>,
    RightValue: FieldElement<P>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum BoolOperation {
    AND,
    OR,
    NOT,
}

impl BoolOperation {
    // Calculate value according to arithemtic formula
    pub fn calculate(&self) -> FieldElement<P> {
        match self {
            Self::AND => LeftValue * RightValue, // (x1 AND x2) = x1*x2
            Self::NOT => 1 - LeftValue,          // NOT(x) = 1 - x
            Self::OR => LeftValue + RightValue - LeftValue * RightValue, // (x1 OR x2) = x1 + x2 - x1 *x2
        }
    }
}
