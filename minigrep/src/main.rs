use log;
use log4rs;
use std::{env, fs};

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let file_path = &args[2];
    (query, file_path)
}

fn main() {
    log4rs::init_file("config/logger.yaml", Default::default()).unwrap();
    log::info!("Minigrep: I/O tool");
    let args: Vec<String> = env::args().collect();
    let (query, file_path) = parse_config(&args);
    log::info!("Searching for `{query}`");
    log::info!("in `{file_path}`");

    let file_content =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
    log::info!("With text: {file_content}");
}
