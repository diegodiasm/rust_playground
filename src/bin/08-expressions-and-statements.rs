fn five() -> i32 {
    5
}


fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    #[allow(unused_must_use)]  // Silence warning.
    let z = {
        let x = 3;
        x + 1;                 // Semi-colon transforms this into statement
    };

    println!("The value of y is: {y}");
    println!("The value of z is: {:?}",z);

    let ret_five = five();

    println!("The value of ret_five is: {ret_five}");
}