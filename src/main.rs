use crate::{examples::toyexample::ToyExample, field::field_element::FieldElement};
use env_logger::Env;
use log::{error, info};

mod examples;
mod field;
mod helpers;
mod polynomials;
mod protocol;

fn main() {
    let env = Env::default().filter_or("RUST_LOG", "info");

    env_logger::init_from_env(env);

    info!("Starting toy example.");

    let toy_example: ToyExample<17> = ToyExample::new();

    if toy_example.run() {
        info!("Toy example ran gracefully.");
        return;
    }

    error!("Toy example ran with failed step.");
}
