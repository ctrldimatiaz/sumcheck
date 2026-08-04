use crate::{examples::toyexample::ToyExample, field::field_element::FieldElement};
use env_logger;

mod examples;
mod field;
mod helpers;
mod polynomial;
mod protocol;

fn main() {
    env_logger::init();

    let toy_example: ToyExample<17> = ToyExample::new();

    if toy_example.run() {
        println!("ToyExample claim successful");
    }
}
