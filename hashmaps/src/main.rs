use log;
use log4rs;
use std::collections::HashMap;

fn main() {
    log4rs::init_file("config/logger.yaml", Default::default()).unwrap();
    log::info!("Hashmaps!");
    log::info!("Creating Hashmaps");
    let mut scores = HashMap::new();
    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("red"), 50);
    log::info!("{:?}", scores);
}
