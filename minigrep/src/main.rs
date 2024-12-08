use log;
use log4rs;
use std::{env, error::Error, fs, process};

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

    if let Err(e) = run(config) {
        log::error!("{}", e);
        process::exit(1);
    }
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

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_content = fs::read_to_string(config.file_path)?;
    log::info!("With text:\n{file_content}");
    Ok(())
}
