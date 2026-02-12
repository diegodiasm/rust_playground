fn main() {
    let mut s = String::from("hello");

    println!("{s}");

    let r1 = &mut s;
    change(r1);

    println!("{r1}");

    let r2 = &mut s;
    change(r2);

    println!("{s}");
}

fn change(mut_ref: &mut String) {
    mut_ref.push_str(", world");
}