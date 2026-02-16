fn main() {
    // The member to be changed has to be declared as mutable ('mut').
    let mut mutable_s = String::from("hello");

    println!("prior change: {mutable_s}");

    // The value shall be passed as a 'mutable reference', i.e. '&mut'.
    change(&mut mutable_s);

    // Because no ownership was given, 's' is still valid here.
    println!("after change: {mutable_s}");

    let immutable_s = String::from("hello");

    // cannot borrow `immutable_s` as mutable, as it is not declared as mutable
    // change(&mut immutable_s); // This will not compile
}

// Next function can mutate the argument, but does so using borrowing:
// this does not demand ownership.
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}