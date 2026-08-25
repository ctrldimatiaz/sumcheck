use crate::{field::field_element::FieldElement, helpers::error::InputParserError};

// Parser for input as described in Exercise 3.4
pub struct MleInput<const P: u64> {
    evaluations: Vec<FieldElement<P>>,
    vector: Vec<FieldElement<P>>,
}

impl<const P: u64> MleInput<P> {
    pub fn new(evaluations: &str, vector: &str) -> Result<Self, InputParserError> {
        Self::mle_input_parser(evaluations, vector)
    }

    // Used to parse array of 2^l length with the f evaluations and the vector r.
    fn mle_input_parser(
        evaluations_input: &str,
        vector_input: &str,
    ) -> Result<Self, InputParserError> {
        if evaluations_input.len() as u64 != 2_u64.pow(vector_input.len() as u32) {
            return Err(InputParserError::LengthMismatch);
        }

        let evaluations_parse: Result<Vec<FieldElement<P>>, InputParserError> = evaluations_input
            .split(',')
            .map(|s| s.trim().parse::<FieldElement<P>>().map_err(|e| e))
            .collect();

        let vector_parse: Result<Vec<FieldElement<P>>, InputParserError> = vector_input
            .split(',')
            .map(|s| s.trim().parse::<FieldElement<P>>().map_err(|e| e))
            .collect();

        let evaluations = evaluations_parse?;
        let vector = vector_parse?;

        Ok(Self {
            evaluations,
            vector,
        })
    }
}
