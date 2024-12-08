use log;
use log4rs;
use std::env;

fn main() {
    log4rs::init_file("config/logger.yaml", Default::default()).unwrap();
    log::info!("Minigrep: I/O tool");
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let file_path = &args[2];
    log::info!("Searching for `{query}` in `{file_path}`");
}
