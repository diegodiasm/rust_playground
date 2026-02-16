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
    let s1 = Tracked(String::from("hello"));

    // Uses a tuple here to take back ownership of the
    // string.
    let (s2, len) = calculate_length(s1);

    println!("The length of '{:?}' is {len}.",s2.0);
}

fn calculate_length(s: Tracked) -> (Tracked, usize) {
    let length = s.0.len(); // len() returns the length of a String

    (s, length)
}