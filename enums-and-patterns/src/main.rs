use log;
use log4rs;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

// #[derive(Debug)]
// enum IpAddr {
//     V4(u8, u8, u8, u8),
//     V6(String),
// }

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// Struct equivalents to the one enum
#[derive(Debug)]
struct QuitMessage; // unit struct

#[derive(Debug)]
struct MoveMessage {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct WriteMessage(String); // tuple struct

#[derive(Debug)]
struct ChangeColorMessage(i32, i32, i32); // tuple struct

fn main() {
    log4rs::init_file("config/logger.yaml", Default::default()).unwrap();
    log::info!("Enums");

    // let home = IpAddr::V4(127, 0, 0, 1);
    // let loopback = IpAddr::V6(String::from("::1"));
    let home = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let loopback = IpAddr::V6(Ipv6Addr::new(234, 2344, 324, 3445, 3435, 3435, 1, 55));

    log::debug!("{:?}", home);
    log::debug!("{:?}", loopback);

    let e_quit = Message::Quit;
    let e_move = Message::Move { x: 12, y: 30 };
    let e_write = Message::Write(String::from("I am moving"));
    let e_change_color = Message::ChangeColor(120, 120, 120);
    log::debug!("enum: {:?}", e_quit);
    log::debug!("enum: {:?}", e_move);
    log::debug!("enum: {:?}", e_write);
    log::debug!("enum: {:?}", e_change_color);

    let s_quit = QuitMessage;
    let s_move = MoveMessage { x: 12, y: 30 };
    let s_write = WriteMessage(String::from("I am moving"));
    let s_change_color = ChangeColorMessage(120, 120, 120);
    log::debug!("struct: {:?}", s_quit);
    log::debug!("struct: {:?}", s_move);
    log::debug!("struct: {:?}", s_write);
    log::debug!("struct: {:?}", s_change_color);
}
