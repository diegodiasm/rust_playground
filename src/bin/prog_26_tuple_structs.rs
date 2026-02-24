#[allow(dead_code)]

#[derive(Debug)]
#[derive(PartialEq)]

// Tuple structs have the added meaning the struct name provides but don’t
// have names associated with their fields; rather, they just have the types
// of the fields.
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