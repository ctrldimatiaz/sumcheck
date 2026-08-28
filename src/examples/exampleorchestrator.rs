use itertools::Itertools;
use log::{error, info};
use std::{io, process::exit};

use crate::{
    examples::{
        mleevaluation::MultilinearExtensionFromEvaluations,
        mleexample::MultilinearExtensionExample, toyexample::ToyExample,
    },
    field::field_element::FieldElement,
    helpers::mleinput::MleInput,
    polynomials::{monomial::Monomial, polynomial::Polynomial},
};

pub struct ExampleOrchestrator<const P: u64> {
    toyexample: ToyExample<P>,
}

impl<'a, const P: u64> ExampleOrchestrator<P> {
    pub fn new(toyexample: ToyExample<P>) -> Self {
        Self { toyexample }
    }

    pub fn run(&self) {
        Self::print_options();
        let mut input = String::new();

        while input.trim() != String::from("0") {
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            // Parse string to integer (e.g., i32)
            let number: i32 = input.trim().parse().expect("Not a valid integer");

            match number {
                1 => {
                    self.toyexample.run();
                }
                2 => {
                    // x2 + 2x1

                    println!("Insert evaluations array (ex.: 1,2,4,8):");

                    let mut evaluations = String::new();

                    io::stdin()
                        .read_line(&mut evaluations)
                        .expect("Failed to read evaluations");

                    println!("Insert vector r (ex.: 1,2,4,8):");

                    let mut vector_r = String::new();

                    io::stdin()
                        .read_line(&mut vector_r)
                        .expect("Failed to read vector r");

                    let parser = match MleInput::<P>::new(&evaluations, &vector_r) {
                        Ok(p) => p,
                        Err(e) => {
                            log::error!("Failed to parse evaluations or vector input: {e}");
                            break;
                        }
                    };

                    let evaluater = MultilinearExtensionFromEvaluations::new(parser);

                    match evaluater.generate_f_tilde() {
                        Ok(mle) => {
                            info!(
                                "Successfully generated f tilde from evaluations and vector. F_tilde: {}",
                                mle
                            );
                            match mle.evaluate(evaluater.get_values()) {
                                Ok(result) => {
                                    info!(
                                        "Evaluated f_tilde at ({}): {}",
                                        evaluater.get_values().iter().join(" , "),
                                        result
                                    )
                                }
                                Err(e) => error!(
                                    "Error evaluating f_tilde at ({}). Error {}",
                                    evaluater.get_values().iter().join(" , "),
                                    e
                                ),
                            }
                        }
                        Err(e) => error!(
                            "Error generating f_tilde from evaluations and vector: {}",
                            e
                        ),
                    }
                }
                3 => {
                    // x2 + 2x1
                    let polynomial: Polynomial<P> = Polynomial::new(vec![
                        Monomial::new(FieldElement::from_u64(1), vec![0, 1]).unwrap(),
                        Monomial::new(FieldElement::from_u64(2), vec![1, 0]).unwrap(),
                    ])
                    .unwrap();
                    let mle_example: MultilinearExtensionExample<P> =
                        MultilinearExtensionExample::new(&polynomial);

                    if mle_example.generate_f_tilde() {
                        info!(
                            "Successfully generated f tilde of polynomial: {}",
                            &polynomial
                        );
                    }
                }
                0 => exit(-1),
                _ => {
                    println!("Not a valid option.");
                }
            }

            input.clear();
            Self::print_options();
        }
    }

    fn print_options() {
        println!(
            "Please insert the example to be tested: \n 1 - ToyExample\n 2 - Multilinear extension through evaluations and vector r (Exercise 3.4)\n 3 - Dummy f tilde generation\n 0 - Exit\n"
        );
    }
}
