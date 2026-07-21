use crate::field::field_element::FieldElement;
use env_logger;

mod field;

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
}
