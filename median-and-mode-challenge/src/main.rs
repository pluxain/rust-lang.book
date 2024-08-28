use log;
use log4rs;
use std::io::stdin;

fn main() {
    log4rs::init_file("config/logger.yaml", Default::default()).unwrap();
    log::info!("Median and Mode!");
    // @see guessing_game
    println!("Please enter an integer list separated by spaces:");
    let mut input = String::new();
    let mut numbers = Vec::new();
    stdin()
        .read_line(&mut input) // read line from io into a mutable reference `&mut var`
        .expect("Failed to read line");

    for i in input.split_whitespace() {
        numbers.push(i);
    }
    log::info!("{numbers:?}");
}
