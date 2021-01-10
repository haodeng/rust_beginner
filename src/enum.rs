#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

// We can represent the same concept in a more concise way using just an enum,
// rather than an enum inside a struct, by putting data directly into each enum variant.
#[derive(Debug)]
enum IpAddrV2 {
    V4(String),
    V6(String),
}

// There’s another advantage to using an enum rather than a struct:
// each variant can have different types and amounts of associated data.
#[derive(Debug)]
enum IpAddrV3 {
    V4(u8, u8, u8, u8),
    V6(String),
}

// this one has a wide variety of types embedded in its variants.
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// just as we’re able to define methods on structs using impl,
// we’re also able to define methods on enums
impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

fn route(ip_kind: IpAddrKind) {
    println!("route: {:?}", ip_kind);
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    println!("{:?}, {:?}", four, six);

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    println!("{:?}, {:?}", home, loopback);

    let home = IpAddrV2::V4(String::from("127.0.0.1"));
    let loopback = IpAddrV2::V6(String::from("::1"));
    println!("{:?}, {:?}", home, loopback);

    let home = IpAddrV3::V4(127, 0, 0, 1);
    let loopback = IpAddrV3::V6(String::from("::1"));
    println!("{:?}, {:?}", home, loopback);

    let m = Message::Write(String::from("hello"));
    m.call();
}