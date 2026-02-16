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

#[allow(unused_variables)]
fn main() {
    let s1 = gives_ownership();        // gives_ownership moves its return
                                       // value into s1

    let s2 = Tracked(String::from("A1"));    // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into
                                       // takes_and_gives_back, which also
                                       // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> Tracked {      // gives_ownership will move its
                                       // return value into the function
                                       // that calls it

    let some_tracked = Tracked(String::from("A2")); // some_tracked comes into scope

    some_tracked                       // some_tracked is returned and
                                       // moves out to the calling
                                       // function
}

// This function takes a String and returns a String.
fn takes_and_gives_back(a_tracked: Tracked) -> Tracked {
    // a_tracked comes into
    // scope

    a_tracked  // a_tracked is returned and moves out to the calling function
}