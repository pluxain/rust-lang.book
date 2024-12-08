use log;
use log4rs;
use std::{env, process};

use minigrep::Config;

fn main() {
    log4rs::init_file("config/logger.yaml", Default::default()).unwrap();
    log::info!("Minigrep: I/O tool");

    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        log::error!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    log::info!("Searching for `{}`", config.query);
    log::info!("in `{}`", config.file_path);

    if let Err(e) = minigrep::run(config) {
        log::error!("{}", e);
        process::exit(1);
    }
}
