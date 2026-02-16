
// Just as variables are immutable by default, so are references.

// Having a reference to a mutable variable is not enough to have the
// permission to modify that variable. We need the reference itself to
// be declared as mutable, as illustrated in prog_17_modify_borrowed_value.rs

fn main() {
    // The variable is declared as mutable.
    let mut s = String::from("hello");

    change(&s);
}

// But the reference is not declared as mutable! So, it cannot be used to
// mutate what it refers to.
fn change(some_string: &String) {
    some_string.push_str(", world");
}
