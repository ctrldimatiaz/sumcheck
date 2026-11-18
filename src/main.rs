use crate::{
    examples::{
        dishonestprover::DishonestProver, exampleorchestrator::ExampleOrchestrator,
        toyexample::ToyExample,
    },
    field::field_element::FieldElement,
};
use dotenvy::dotenv;
use env_logger::Env;

mod examples;
mod field;
mod helpers;
mod polynomials;
mod protocol;

const PRIMEP: u64 = 17;

fn main() {
    // load .env
    dotenv().ok();

    let env = Env::default().filter_or("RUST_LOG", "info");

    env_logger::init_from_env(env);

    // Initialize examples
    let toy_example: ToyExample<PRIMEP> = ToyExample::new();
    let dishonest_prover: DishonestProver<PRIMEP> = DishonestProver::new();

    // Iniitialize and run orchestrator
    let exampleorchestrator = ExampleOrchestrator::new(toy_example, dishonest_prover);

    exampleorchestrator.run();
}
