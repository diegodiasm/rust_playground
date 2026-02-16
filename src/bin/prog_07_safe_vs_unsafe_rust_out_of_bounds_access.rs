use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    // Explicitly opt out of safety: use `get_unchecked()`
    // Being in unsafe does not automatically turn off safety checks.
    // It only allows you to call unsafe operations explicitly.
    unsafe {
      let element = *a.get_unchecked(index);
      println!("UNSAFE (Line {}): The value of the element at index {index} is: {element}.",line!()-1);
    }
    // Safe Rust cannot cause memory unsafety. The next line
    // will always panick if the index is out of bounds.
    let element = a[index];
    println!("SAFE   (Line {}): The value of the element at index {index} is: {element}.",line!()-1);
}