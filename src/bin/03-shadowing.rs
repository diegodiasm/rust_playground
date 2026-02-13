fn main() {
    let x = 5;

    let x = x + 1; // x==6

    {
        let x = x * 2; // x==12
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}"); // x==6
}