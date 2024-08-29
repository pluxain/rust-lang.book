use log;
use log4rs;
use std::io::stdin;

fn main() {
    log4rs::init_file("config/logger.yaml", Default::default()).unwrap();
    log::info!("Median and Mode!");
    // @see guessing_game
    let mut numbers: Vec<i32>;

    'prompt: loop {
        println!("Please enter a number list separated by spaces:");
        let mut input = String::new();
        numbers = vec![];
        stdin()
            .read_line(&mut input) // read line from io into a mutable reference `&mut var`
            .expect("Failed to read line");

        for s in input.split_whitespace() {
            let number: i32 = match s.parse() {
                Ok(n) => n,
                Err(_) => {
                    log::error!("Wrong input by user {}({})", input, s);
                    println!("Please enter a number list!");
                    continue 'prompt;
                }
            };
            numbers.push(number);
        }
        break 'prompt;
    }
    numbers.sort();
    log::info!("{numbers:?}");
    log::info!(
        "The median value of the list [{}] is {}",
        numbers
            .iter()
            .map(|&n| n.to_string())
            .collect::<Vec<String>>()
            .join(", "),
        numbers.get(numbers.len() / 2).unwrap()
    );
    // TODO: display mode
}
