use std::cell::RefCell;

fn main() {
    let x = RefCell::new(10);

    {
        let mut r = x.borrow_mut(); // mutable borrow at runtime
        *r += 1;
        println!("inside borrow_mut: {}", *r);
    } // r goes out of scope → mutable borrow ends

    println!("after: {}", x.borrow());
}