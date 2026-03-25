fn main() {

    // Rust can infer these types because we've
    // specified a value inside the Some variant.
    let some_number = Some(5);
    let some_char = Some('e');

    // The compiler can’t infer the type that the corresponding Some variant
    // will hold by looking only at a None value.
    let absent_number: Option<i32> = None;

    let value_5 = some_number.unwrap();
    let trigger_panic = absent_number.unwrap_or(0);
    let trigger_panic = absent_number.unwrap();

}


