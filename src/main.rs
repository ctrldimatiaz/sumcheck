use crate::{
    field::field_element::FieldElement,
    polynomial::{
        monomial::Monomial, multilinear::MultilinearPolynomial,
        multivariate::MultivariatePolynomial, polynomial::Polynomial,
        univariate::UnivariatePolynomial,
    },
};
use env_logger;

mod error;
mod field;
mod polynomial;

fn main() {
    env_logger::init();

    //after initialization there are no negative field elements
    let field_element: FieldElement<17> = FieldElement::from_u64(6);
    let negative_field_element: FieldElement<17> = FieldElement::from_i64(-55);

    println!("Hello algebraic element {}", field_element.value);
    println!(
        "Hello negative algebraic element {}",
        negative_field_element
    );

    let field_element_operation: FieldElement<17> = field_element + negative_field_element;
    println!(
        "Hello algebraic element addition: {}",
        field_element_operation
    );

    let field_element_subtract: FieldElement<17> = field_element - negative_field_element;
    println!(
        "Hello algebraic element subtracted: {}",
        field_element_subtract
    );

    let field_element_multiplied: FieldElement<17> = field_element * negative_field_element;
    println!(
        "Hello algebraic element multiplied: {}",
        field_element_multiplied
    );

    let field_element_divided: FieldElement<17> = field_element / negative_field_element;
    println!("Hello algebraic element divided: {}", field_element_divided);

    let default_element: FieldElement<17> = FieldElement::default();
    println!("Hello default value: {}", default_element);

    let vector: Vec<FieldElement<17>> =
        vec![field_element, negative_field_element, field_element_divided];

    let polynomial = UnivariatePolynomial::from_field_vec(vector);

    let result = polynomial.evaluate(&field_element_multiplied);

    println!(
        "Evaluate polynomial {} {}x {}x² at element {}: Result : {}",
        field_element,
        negative_field_element,
        field_element_divided,
        field_element_multiplied,
        result
    );

    let variables_vector: Vec<Vec<FieldElement<17>>> = vec![
        vec![field_element, field_element_divided, field_element_subtract],
        vec![
            field_element_subtract,
            field_element_subtract,
            field_element_subtract,
        ],
    ];

    let multivariate_polynomial = MultivariatePolynomial::from_variables_vec(variables_vector);

    let result = multivariate_polynomial.evaluate(&vec![field_element, field_element_divided]);

    println!(
        "Evaluate polynomial {} {}x{}y {}x²{}y² at element (x,y) {} {}: Result : {}",
        field_element * field_element_subtract,
        field_element_divided,
        field_element_subtract,
        field_element_subtract,
        field_element_subtract,
        field_element,
        field_element_divided,
        result
    );

    let polynomial: Polynomial<17> = Polynomial::new(vec![
        Monomial {
            coefficient: FieldElement::from_u64(5),
            exponents: vec![0, 0],
        },
        Monomial {
            coefficient: FieldElement::from_u64(5),
            exponents: vec![1, 1],
        },
    ])
    .unwrap();

    let multilinear = MultilinearPolynomial::new(polynomial).unwrap();

    let result = multilinear
        .evaluate(vec![field_element_subtract, field_element_divided])
        .unwrap();

    println!("Evaluate multilinear polynomial at  Result : {}", result);
}
