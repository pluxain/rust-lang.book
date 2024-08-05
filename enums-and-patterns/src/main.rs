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

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
    Colorado,
    Connecticut,
    Delaware,
    Florida,
    Georgia,
    Hawaii,
    Idaho,
    Illinois,
    Indiana,
    Iowa,
    Kansas,
    Kentucky,
    Louisiana,
    Maine,
    Maryland,
    Massachusetts,
    Michigan,
    Minnesota,
    Mississippi,
    Missouri,
    Montana,
    Nebraska,
    Nevada,
    NewHampshire,
    NewJersey,
    NewMexico,
    NewYork,
    NorthCarolina,
    NorthDakota,
    Ohio,
    Oklahoma,
    Oregon,
    Pennsylvania,
    RhodeIsland,
    SouthCarolina,
    SouthDakota,
    Tennessee,
    Texas,
    Utah,
    Vermont,
    Virginia,
    Washington,
    WashingtonDC,
    WestVirginia,
    Wisconsin,
    Wyoming,
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

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

    // Option
    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;
    log::debug!("Option<i32> initialized with Some(5): {:?}", some_number);
    log::debug!("Option<Char> initialized with Some('e'): {:?}", some_char);
    log::debug!(
        "Option<i32> initialized with None needs a declared type: {:?}",
        absent_number
    );

    let _x: i8 = 5;
    let _y: Option<i8> = Some(5);

    // let sum = _x + _y; -> won't work as compiler is not sure Option<i8> contains a value

    let penny = Coin::Penny;
    log::debug!(
        "Value in cents for a {:?} is {:?}",
        Coin::Penny,
        value_in_cents(penny)
    );

    let nickel = Coin::Nickel;
    log::debug!(
        "Value in cents for a {:?} is {:?}",
        Coin::Nickel,
        value_in_cents(nickel)
    );
    let dime = Coin::Dime;

    log::debug!(
        "Value in cents for a {:?} is {:?}",
        Coin::Dime,
        value_in_cents(dime)
    );

    let quarter = Coin::Quarter(UsState::California);
    log::debug!(
        "Value in cents for a {:?} is {:?}",
        Coin::Quarter(UsState::California),
        value_in_cents(quarter)
    );

    // Match with Option<T>
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    log::debug!("plus one of {:?} is {:?}", five, six);
    log::debug!("plus one of {:?} is {:?}", none, plus_one(none));

    // Catch-all Patterns and the _ Placeholder
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    log::debug!("New rule ! Reroll on other than 3 or 7");
    let dice_roll = 7;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }

    log::debug!("New rule ! Do nothing on other than 3 or 7");
    let dice_roll = 3;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            log::debug!("Lucky penny !");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            log::debug!("State quarter from {state:?}!");
            25
        }
    }
}

fn plus_one(n: Option<i32>) -> Option<i32> {
    match n {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn add_fancy_hat() {
    log::debug!("Add fancy hat!");
}

fn remove_fancy_hat() {
    log::debug!("Remove fancy hat!");
}

fn move_player(num_spaces: u8) {
    log::debug!("Move player to {}", num_spaces);
}

fn reroll() {
    log::debug!("Reroll!");
}
