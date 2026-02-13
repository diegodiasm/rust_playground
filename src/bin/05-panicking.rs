// By default, cargo run already runs in debug mode, e.g.
// $ cargo run --bin 05-debug-mode-vs-release-mode

// To run in release mode:
// $ cargo run --release --bin 05-debug-mode-vs-release-mode

fn main() {
  let mut x : u8 = 255;
  let y : u8 = x.wrapping_add (1); // Wrapping is explicitly declared and not
                                   // triggers panicking
  x = x + 1;

  println!("The value of y is {y}");
  println!("The value of x is {x}");
}