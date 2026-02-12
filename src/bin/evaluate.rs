fn main() {

  let somex = Some("foo");
  let nonex : Option<&str> = None;

  assert_eq!(somex.ok_or(0), Ok("foo"));

  println!("The value of somex is: {:?}", somex);
  println!("The value of nonex.ok_or(0) is: {:?}", nonex.ok_or(0));
}