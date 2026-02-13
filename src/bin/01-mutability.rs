fn main() {
    //let y = 5; // Immutable
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    //y = 6; // Compilation error
    println!("The value of x is: {x}");
}