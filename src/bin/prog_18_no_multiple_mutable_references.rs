// Rust rule: at most ONE mutable reference (&mut T) at a time
// The rule is also checked inside UNSAFE blocks!
fn main() {
    let mut s = String::from("hello");
    let r1 = &mut s;
    // Cannot borrow 's' as mutable more than once at a time
    let r2 = &mut s;

    // Reference to 'r1' is crucial: if it was not there, then second
    // borrow would succeed.

    println!("{r1}, {r2}");
}