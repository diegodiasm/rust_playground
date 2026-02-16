// Extra code to notify when dropping occurs.
struct Tracked(String);

// Note: In C++, this pattern of deallocating resources at the end of an item’s
// lifetime is sometimes called Resource Acquisition Is Initialization (RAII).
// The drop function in Rust will be familiar to you if you’ve used RAII
// patterns.


impl Drop for Tracked {
    fn drop(&mut self) {
        println!("dropping {:?}", self.0);
    }
}

fn main() {
    let s = Tracked(String::from("hello"));  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    // takes_ownership(s);             // Compilation error
                                       // value borrowed here after move

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // Because i32 implements the Copy trait,
                                    // x does NOT move into the function,
                                    // so it's okay to use x afterward.
    makes_copy(x);



} // Here, x goes out of scope, then s. However, because s's value was moved,
  // nothing special happens.

fn takes_ownership(some_string: Tracked) { // some_string comes into scope
    println!("takes_ownership: {:?}", some_string.0);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("makes_copy: {some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.