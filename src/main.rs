use crate::field::field_element::FieldElement;

mod field;

fn main() {
    let field_element: FieldElement<17> = FieldElement::from_u64(55);
    let negative_field_element: FieldElement<17> = FieldElement::from_i64(-55);
    println!("Hello algebraic element {}", field_element.value);
    println!(
        "Hello negative algebraic element {}",
        negative_field_element.value
    );

    let field_element_operation: FieldElement<17> = field_element + negative_field_element;
    println!("Hello algebraic element: {}", field_element_operation.value);

    let field_element_subtract: FieldElement<17> = field_element - negative_field_element;
    println!("Hello algebraic element: {}", field_element_subtract.value);
}
