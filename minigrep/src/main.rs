use log;
use log4rs;
use std::{env, fs, process};

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

    run(config);
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Config { query, file_path })
    }
}

fn run(config: Config) {
    let file_content =
        fs::read_to_string(config.file_path).expect("Should have been able to read the file");
    log::info!("With text:\n{file_content}");
}
