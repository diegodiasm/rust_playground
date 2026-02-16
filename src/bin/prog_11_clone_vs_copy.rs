#[allow(unused_mut)]  // Silence warning.
fn main() {

    let mut s1 = String::from("hello");

    // let s2 = s1;              // Copy Traitnot implemented, s1 moved to s2
    // println!("{s1}, world!"); // compilation error: s1 borrowed after move

    let mut s2 = s1.clone();
    println!("s1 = {s1}, s2 = {s2}");

    s2 = String::from("world");
    println!("{s1}, {s2}!");
}
