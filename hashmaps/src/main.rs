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

    log::info!("Accessing values in a Hashmap");
    let team_name = String::from("blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    log::info!("score for {team_name} is {score}.");
    let team_name = String::from("purple");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    log::info!("score for {team_name} is {score}.");
    for (key, value) in scores {
        log::info!("{key}: {value}");
    }
}
