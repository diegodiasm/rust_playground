// Extra code to notify when dropping occurs.
struct Tracked(String);

impl Drop for Tracked {
    fn drop(&mut self) {
        println!("dropping {:?}", self.0);
    }
}

fn main() {
    let s1 = Tracked(String::from("Persistent"));

    // Passing argument: &<VarName>
    let len = calculate_length(&s1);

    // The next reference is created on the fly and not assigned to
    // anything. It is dropped immediately upon return of the function
    // call.
    let len_dropped_ref = calculate_length(&Tracked(String::from("Temp")));
    println!("The length of the dropped reference is {len_dropped_ref}.");

    println!("The length of '{:?}' is {len}.",s1.0);

}

// In idiomatic Rust, functions do not take ownership of their arguments unless
// they need to, and the reasons for that will become clear as we keep going.

// Parameter declared as: &<Type>
fn calculate_length(s: &Tracked) -> usize {
    s.0.len()
} // Here, s goes out of scope. But because s does not have ownership of what
  // it refers to, the Tracked is not dropped.