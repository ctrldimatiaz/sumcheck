use std::{io, process::exit};

use log::info;

use crate::{
    examples::{mleexample::MultilinearExtensionExample, toyexample::ToyExample},
    field::field_element::FieldElement,
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
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        while input.trim() != String::from("0") {
            // Parse string to integer (e.g., i32)
            let number: i32 = input.trim().parse().expect("Not a valid integer");

            println!("Parsed number: {}", number);

            match number {
                1 => {
                    self.toyexample.run();
                    return;
                }
                2 => {
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
                3 => {
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
                0 => exit(-1),
                _ => {
                    println!("Not a valid option.");
                }
            }

            Self::print_options();
        }
    }

    fn print_options() {
        println!(
            "Please insert the example to be tested: \n 1 - ToyExample\n 2 - Multilinear extension through evaluations and vector r (Exercise 3.4)\n 0 - Exit"
        );
    }
}
