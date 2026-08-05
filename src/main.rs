use crate::{examples::toyexample::ToyExample, field::field_element::FieldElement};
use log::{error, info};

mod examples;
mod field;
mod helpers;
mod polynomial;
mod protocol;

fn main() {
    env_logger::init();

    info!("Starting toy example.");

    let toy_example: ToyExample<17> = ToyExample::new();

    if toy_example.run() {
        info!("Toy example ran gracefully.");
        return;
    }

    error!("Toy example ran with failed step.");
}
