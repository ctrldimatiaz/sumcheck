use crate::field::field_element::FieldElement;

// Parser for input as described in Exercise 3.4
pub struct MleInput<const P: u64> {
    evaluations: Vec<FieldElement<P>>,
    vector: Vec<FieldElement<P>>,
}

impl<const P: u64> MleInput<P> {
    pub fn new(evaluations: &str, vector: &str) -> Self {
        Self::mle_input_parser(evaluations, vector)
    }

    // Used to parse array of 2^l length with the f evaluations and the vector r.
    fn mle_input_parser(evaluations_input: &str, vector_input: &str) -> Self {
        let evaluations: Vec<FieldElement<P>> = evaluations_input
            .split(',')
            .map(|s| s.parse::<FieldElement<P>>().expect("Failed to parse"))
            .collect();

        let vector: Vec<FieldElement<P>> = vector_input
            .split(',')
            .map(|s| s.parse::<FieldElement<P>>().expect("Failed to parse"))
            .collect();

        Self {
            evaluations,
            vector,
        }
    }
}
