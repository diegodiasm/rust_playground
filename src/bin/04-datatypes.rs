fn main() {
  let guess: u32 = "42".parse().expect("Not a number!");
  //       ^^^^^^              ^^^^^^^^^^^^^^^^^^^^^^^^
  //      required             what to show at runtime if
  //   type annotation         a failure occurs.
  println!("The value of guess is: {guess}");
}