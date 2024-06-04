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
    ChangeColor(i32, i32, i32),
    Move { x: i32, y: i32 },
    Quit,
    Write(String),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
        log::debug!("call method: {:?}", self);
    }
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

    let e_change_color = Message::ChangeColor(120, 120, 120);
    let e_quit = Message::Quit;
    let e_move = Message::Move { x: 12, y: 30 };
    let e_write = Message::Write(String::from("I am moving"));

    log::debug!("enum: {:?}", e_change_color);
    log::debug!("enum: {:?}", e_move);
    log::debug!("enum: {:?}", e_quit);
    log::debug!("enum: {:?}", e_write);

    e_change_color.call();
    e_move.call();
    e_quit.call();
    e_write.call();

    let s_change_color = ChangeColorMessage(120, 120, 120);
    let s_move = MoveMessage { x: 12, y: 30 };
    let s_quit = QuitMessage;
    let s_write = WriteMessage(String::from("I am moving struc"));

    log::debug!("struct: {:?}", s_change_color);
    log::debug!("struct: {:?}", s_move);
    log::debug!("struct: {:?}", s_quit);
    log::debug!("struct: {:?}", s_write);
}
