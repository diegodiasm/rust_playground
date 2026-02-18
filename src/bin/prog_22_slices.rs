// Problem:
// Write a function that takes a string of words separated by spaces and returns
// the first word it finds in that string. If the function doesn’t find a space
// in the string, the whole string must be one word, so the entire string should
// be returned.

fn main() {

    let mut s = String::from("Hello earth");
    #[allow(unused_mut)]
    let mut t = String::from("Hello mars");
    let len_first_word = length_first_word(&s);

    // No problem in clearing the string: `len_first_word`
    // is independent from `s`.
    s.clear();

    println!("Length of the first word is {len_first_word}");

    s = String::from("Hello earth");

    // Slices:
    //  [starting_index..ending_index], where starting_index is the first
    //  position in the slice and ending_index is «one more than the last
    //  position» in the slice.
    let w1 = &s[0..5];
    let w2 = &s[6..11];

    println!("w1 is: {w1}");
    println!("w2 is: {w2}");

    let word = first_word(&s);

    // Compilation error: cannot borrow as mutable in `s.clear()` because it is
    // already borrowed as immutable by the call in the line above.
    // s.clear();

    println!("First word is: {word}");


    // Next function borrows only the second argument!
    let bt = borrow_second_arg(&s, &t);
    // Lifetime of `s` ends above.

    // We can clear s because only `t` is borrowed.
    s.clear();
    // We cannot clear `t` because it is borrowed as immutable!
    // t.clear();

    println!("Borrowed argument is: {bt}");
}

// This function returns a usize that is only meaningful in the context of
// the &String argument. Because it’s a separate value from the String, there’s
// no guarantee that the reference and this value are kept consistent. The
// return corresponds to the length of the first word.
fn length_first_word(s: &String) -> usize {
    // Convert String to an array of bytes using the as_bytes
    let bytes = s.as_bytes();

    // Create an iterator over the array of bytes using the iter method:
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// Tracking of borrowed parameters is done explictly
#[allow(unused_variables)]
fn borrow_second_arg<'a>(s: & String, t : &'a String) -> &'a str {
    &t[..]
}