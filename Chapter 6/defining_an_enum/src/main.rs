enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}
// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

// enum Option<T> {
//     None,
//     Some(T),
// }
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {}
}
fn main() {
    let four = IpAddr::V4;
    let six = IpAddr::V6;
    println!("Hello, world!");

    let home = IpAddr::V4(12, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));

    // println!("Ip Addresses {} {}", home, loopback);
    println!("\nThe Option Enum and Its Advantages Over Null Values");
    let some_number = Some(5);
    let some_char = Some('R');
    let absent_number: Option<i32> = None;
}

fn route(ip_kind: IpAddr) {}
