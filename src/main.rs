use crate::{
    examples::{exampleorchestrator::ExampleOrchestrator, toyexample::ToyExample},
    field::field_element::FieldElement,
};
use dotenvy::dotenv;
use env_logger::Env;
use log::info;

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

    // start examples
    info!("Starting toy example.");

    let toy_example: ToyExample<PRIMEP> = ToyExample::new();
    let exampleorchestrator = ExampleOrchestrator::new(toy_example);

    exampleorchestrator.run();
}
