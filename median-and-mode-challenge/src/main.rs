use log;
use log4rs;
use std::collections::HashMap;
use std::io::stdin;

fn main() {
    log4rs::init_file("config/logger.yaml", Default::default()).unwrap();
    log::info!("Challenge: Median and Mode!");
    // @see guessing_game
    let mut numbers: Vec<i32>;
    let mut count: HashMap<i32, u32>;

    'prompt: loop {
        println!("Please enter a number list separated by spaces:");
        let mut input = String::new();
        numbers = vec![];
        count = HashMap::new();
        stdin()
            .read_line(&mut input) // read line from io into a mutable reference `&mut var`
            .expect("Failed to read line");

        for s in input.split_whitespace() {
            let number: i32 = match s.parse() {
                Ok(n) => {
                    let count = count.entry(n).or_insert(0);
                    *count += 1;
                    n
                }
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
    log::info!("Collected list of numbers: {numbers:?}");
    let list_as_s = numbers
        .iter()
        .map(|&n| n.to_string())
        .collect::<Vec<String>>()
        .join(", ");
    numbers.sort();
    println!(
        "The median value of the list [{}] is {}",
        list_as_s,
        numbers.get(numbers.len() / 2).unwrap()
    );
    log::info!("Collected HashMap for count: {count:?}");
    // Transform hashmap to vector or tuple
    let mut count_vec: Vec<(&i32, &u32)> = count.iter().collect();
    count_vec.sort_by(|a, b| b.1.cmp(a.1));
    log::info!(
        "Count HashMap converted to Vector or Tuples: {:?}",
        count_vec
    );
    let mode = count_vec.get(0).unwrap();
    println!(
        "The mode of the list {} is {} with {} counts",
        list_as_s, mode.0, mode.1
    )
}
