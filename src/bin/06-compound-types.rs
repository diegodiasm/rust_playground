fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    let unit_tuple = ();

    let a = [1, 2, 3, 4, 5];

    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];


    // Explicitly declare the type of the array: [ELEM_TYPE; SIZE]
    let explictly_typed_a: [i32; 5] = [1, 2, 3, 4, 5];

    println!("Tuple tup is ({x}, {y}, {z}).");
    println!("Tuple tup is ({}, {}, {}).", tup.0,tup.1,tup.2);
    println!("Tuple tup is {:?}.", tup);           // Debug Trait
    println!("Unit tuple is {:?}.", unit_tuple);   // Debug Trait
    println!("The array a is {:?}", a);            // Debug Trait
    println!("The array months is {:?}", months);  // Debug Trait
    println!("The array explictly_typed_a is {:?}", explictly_typed_a);
}