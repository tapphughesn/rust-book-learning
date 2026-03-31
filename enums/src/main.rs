#![allow(unused)]
enum IpAddrKind {
    V4(String),
    V6(String),
}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32,i32,i32),
}

fn route(ip_kind: IpAddrKind) {}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    println!("Hello, world!");
}
