// Derive the Debug trait
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
#[allow(dead_code)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}


#[allow(unused_variables)]
// Enumerations are treated as resources: the borrow checker tracks ownership!
fn route(ip_kind: IpAddrKind) {}


#[derive(Debug)]
#[allow(dead_code)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("Displaying: {self:?}");
    }
}


#[allow(unused_variables)]
fn main() {

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    println!("ipv4_addr={four:?} ipv6_addr={six:?}");

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
    route(six);
    //route(six); // Ownership already taken by function call above.

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    println!("home={home:?} loopback={loopback:?}");

    let m = Message::Write(String::from("hello"));
    m.call();
}