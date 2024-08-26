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
}
