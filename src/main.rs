use std::env;

use crate::{
    examples::{
        exampleorchestrator::ExampleOrchestrator, mleexample::MultilinearExtensionExample,
        toyexample::ToyExample,
    },
    field::field_element::FieldElement,
    polynomials::{monomial::Monomial, polynomial::Polynomial},
};
use dotenvy::dotenv;
use env_logger::Env;
use log::{error, info};

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
