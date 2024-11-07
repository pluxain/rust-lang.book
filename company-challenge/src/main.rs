// Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company;
// for example, “Add Sally to Engineering” or “Add Amir to Sales.”
// Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.

use log;
use log4rs;
use std::io::{self, stdin};

#[derive(Debug)]
enum Command {
    Add,
    Remove,
}

fn prompt(p: &str) -> Result<String, io::Error> {
    log::info!("Prompting");
    let mut input = String::new();
    println!("{p}");
    match stdin().read_line(&mut input) {
        Ok(_) => Ok(input.trim().to_string()),
        Err(err) => Err(err),
    }
}

fn main() {
    log4rs::init_file("config/logger.yaml", Default::default()).unwrap();
    log::info!("challenge: company");

    loop {
        let input = prompt(
            "What do you want to do (ex: `Add Robert to DevOps`, `Remove Wendy from Pirates`)",
        )
        .unwrap();
        log::info!("input is `{input}`");
        let mut iter = input.split_whitespace();

        let command = match iter.next().unwrap().to_lowercase() {
            _c if _c == String::from("add") => Command::Add,
            _c if _c == String::from("remove") => Command::Remove,
            _ => {
                println!(
                    "Please use a valid command `{:?}` or `{:?}`",
                    Command::Add,
                    Command::Remove
                );
                continue;
            }
        };
        log::info!("command is `{command:?}`");

        let name = iter.next().unwrap();
        log::info!("name is `{name}`");
        break;
    }
}
