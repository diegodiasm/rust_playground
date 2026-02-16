// Rust rule:

// Either:
//   many immutable references (&T)
// or:
//   exactly ONE mutable reference (&mut T)
// But never both at the same time!

// The rule is also checked inside UNSAFE blocks!
fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

    println!("{r1}, {r2}, and {r3}");
}