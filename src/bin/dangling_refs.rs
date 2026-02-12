fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> String {
    let s = String::from("hello");

    s
}

// fn main() {
//     let ds = String::from("hello");
//     let _reference_to_nothing = dangle(&ds);
// }

// fn dangle(_ds : &String ) -> &String {
//     let s = String::from("hello");

//     &s
// }