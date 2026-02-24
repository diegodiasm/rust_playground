// Derive the Debug trait
#[derive(Debug)]
#[allow(dead_code)]
struct Rectangle {
    width: u32,
    height: u32,
}


fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    //println!("rect1 is {rect1:#?}");
    println!("rect1 is {rect1:?}");
    println!("The area of the rectangle is {} square pixels.", rect1.area());

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    let square = Rectangle::square(30);

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("Can rect1 hold square? {}", rect1.can_hold(&square));
}

impl Rectangle {
    // Methods must have a parameter named «self» of type «Self» for their first
    // parameter, so Rust lets you abbreviate this with only the name self in
    // the first parameter spot. Within an impl block, the type Self is an alias
    // for the type that the impl block is for.
    // Thus: «area(&self)» = «area(self : &Self)» = «area(self : &Rectangle)»
    // No overloading for self is allowed!
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // We can give a method the same name as one of the struct's fields.
    fn width(&self) -> bool {
        self.width > 0
    }
}

// Multiple impl blocks referencing the same struct are allowed.
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }

    // Associated functions that aren't methods
    // The Self keywords in the return type and in the body of the function are
    // aliases for the type that appears after the impl keyword, which in this
    // case is Rectangle.
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}