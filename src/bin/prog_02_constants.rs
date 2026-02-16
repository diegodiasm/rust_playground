// Rust's naming convention for constants is to use all uppercase with
// underscores between words.
//
// Subset of expression evaluation available for constant declaration:
// https://doc.rust-lang.org/reference/const_eval.html
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    println!("The value of THREE_HOURS_IN_SECONDS is: {THREE_HOURS_IN_SECONDS}");
}

