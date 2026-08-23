use crate::{
    examples::{mleexample::MultilinearExtensionExample, toyexample::ToyExample},
    field::field_element::FieldElement,
    polynomials::{monomial::Monomial, multilinear::MultilinearPolynomial, polynomial::Polynomial},
};
use dotenvy::dotenv;
use env_logger::Env;
use log::{error, info};

mod examples;
mod field;
mod helpers;
mod polynomials;
mod protocol;

fn main() {
    // load .env
    dotenv().ok();

    let env = Env::default().filter_or("RUST_LOG", "info");

    env_logger::init_from_env(env);

    // start examples
    info!("Starting toy example.");

    let toy_example: ToyExample<17> = ToyExample::new();

    if toy_example.run() {
        info!("Toy example ran gracefully.");

        // x2 + 2x1
        let polynomial: Polynomial<17> = Polynomial::new(vec![
            Monomial::new(FieldElement::from_u64(1), vec![0, 1]).unwrap(),
            Monomial::new(FieldElement::from_u64(2), vec![1, 0]).unwrap(),
        ])
        .unwrap();
        let mle_example: MultilinearExtensionExample<17> =
            MultilinearExtensionExample::new(&polynomial);

        if mle_example.generate_f_tilde() {
            info!(
                "Successfully generated f tilde of polynomial: {}",
                &polynomial
            );
            return;
        }
    }

    error!("Toy example ran failed.");
}
