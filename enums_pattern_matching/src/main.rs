fn main() {

    // let four = IpAddrKind::V4;
    // let five = IpAddrKind::V6;

    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };

    // let loopback = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1"),
    // }

    let home = IpAddr::V4(12, 12, 12, 12);

    let loop_back = IpAddr::V6(String::from("::1"));

    println!("Hello, world!");
}

// struct IpAddr {
//     kind: IpAddrKind,
//     address: String
// }

// enum IpAddrKind {
//     V4, V6
// }

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}