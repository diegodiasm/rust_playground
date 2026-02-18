
fn main() {

    // String literals have type &str
    let s : &str = "Hello earth!";
    let t = String::from("Hello mars!");


    println!("first_word(s)        = {:?}", first_word(s));
    println!("first_word(&s[6..])  = {:?}", first_word(&s[6..]));
    println!("first_word(&s[..])   = {:?}", first_word(&s[..]));
    println!("first_word(&t)       = {:?}", first_word(&t));
    println!("first_word(t[6..11]) = {:?}", first_word(&t[6..11]));
}


// String Slices as Parameters: improved signature
//  fn first_word(s: &String) -> &sts
//  fn first_word(s: &str)    -> &sts
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}