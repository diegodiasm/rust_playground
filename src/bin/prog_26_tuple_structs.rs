#[allow(dead_code)]

#[derive(Debug)]
#[derive(PartialEq)]
struct Color(i32, i32, i32);

#[derive(Debug)]
#[derive(PartialEq)]
struct Point(i32, i32, i32);

#[allow(unused_variables)]
fn main() {
  let black = Color(0, 0, 0);
  let origin = Point(0, 0, 0);

  // Next line is a compilation error: black and origin values are of different
  // types because they're instances of different tuple structs.
  // println!("{}", (black == origin));

  // Destructuring values in tuple structs
  let Point(x, y, z) = origin;

  println!("Point({x},{y},{z})");
}