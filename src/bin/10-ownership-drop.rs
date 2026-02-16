// Extra code to notify when dropping occurs.
struct Tracked(String);

// Note: In C++, this pattern of deallocating resources at the end of an item’s
// lifetime is sometimes called Resource Acquisition Is Initialization (RAII).
// The drop function in Rust will be familiar to you if you’ve used RAII
// patterns.


impl Drop for Tracked {
    fn drop(&mut self) {
        println!("Tracked value is being dropped: {}", self.0);
    }
}

fn main() {
    {
        let s = Tracked(String::from("hello"));
        println!("inside scope");
    }
    println!("outside scope");
}