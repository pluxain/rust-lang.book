use log;
use log4rs;
use std::{env, fs};

struct Config {
    query: String,
    file_path: String,
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let file_path = args[2].clone();
    Config { query, file_path }
}

fn main() {
    log4rs::init_file("config/logger.yaml", Default::default()).unwrap();
    log::info!("Minigrep: I/O tool");

    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args);

    log::info!("Searching for `{}`", config.query);
    log::info!("in `{}`", config.file_path);

    let file_content =
        fs::read_to_string(config.file_path).expect("Should have been able to read the file");
    log::info!("With text: {file_content}");
}
