use log;
use log4rs;
use std::collections::HashMap;

fn main() {
    log4rs::init_file("config/logger.yaml", Default::default()).unwrap();
    log::info!("HashMaps!");
    log::info!("Creating HashMaps");
    let mut scores = HashMap::new();
    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("red"), 50);
    log::info!("{:?}", scores);

    log::info!("Accessing values in a HashMap");
    let team_name = String::from("blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    log::info!("score for {team_name} is {score}.");
    let team_name = String::from("purple");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    log::info!("score for {team_name} is {score}.");
    for (key, value) in scores {
        log::info!("{key}: {value}");
    }

    log::info!("HashMaps and ownership");
    let field_name = String::from("favourite color");
    let field_value = String::from("blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // log::error!("{field_name} is {field_value}"); // -> fails as the map has taken ownership of the Strings.
    // log::info!(
    //     "{} is {}",
    //     field_name,
    //     map.get(&field_name).unwrap_or(&String::from("unknown"))
    // ); // -> fails as well
    log::info!(
        "{} is {}",
        String::from("favourite color"),
        map.get(&String::from("favourite color"))
            .unwrap_or(&String::from("unknown"))
    );

    log::info!("Updating a HashMap");
    log::info!("Overwriting a value");
    let mut scores = HashMap::new();
    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("blue"), 25);
    log::info!("{scores:?}");
    log::info!("Adding a Key and Value only if Key is not present");
    let mut scores = HashMap::new();
    scores.insert(String::from("blue"), 10);
    scores.entry(String::from("yellow")).or_insert(50);
    scores.entry(String::from("blue")).or_insert(50);
    log::info!("{scores:?}");
    log::info!("Updating a Value based on the old Value");
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    log::info!("{map:?}");
}
