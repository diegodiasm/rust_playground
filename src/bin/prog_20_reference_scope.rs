// The scopes of the immutable references r1 and r2 end after the println!
// where they are last used, which is before the mutable reference r3 is
// created. These scopes don't overlap, so this code is allowed: The compiler
// can tell that the reference is no longer being used at a point before the
// end of the scope.

fn main() {

  let mut s = String::from("hello");

  let r1 = &s; // no problem
  let r2 = &s; // no problem
  println!("{r1} and {r2}");

  // Variables r1 and r2 will not be used after this point.
  let r3 = &mut s; // no problem
  println!("{r3}");
}