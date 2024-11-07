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

struct Action {
    command: Option<Command>,
    name: String,
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

fn parse(input: String) -> Action {
    let parts: Vec<&str> = input.split_whitespace().collect();
    let command = match parts.get(0).unwrap().to_lowercase() {
        _c if _c == String::from("add") => Some(Command::Add),
        _c if _c == String::from("remove") => Some(Command::Remove),
        _ => None,
    };
    let name = parts.get(1).unwrap().to_string();
    Action { command, name }
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

        let Action { command, name } = parse(input);
        log::info!("command is `{command:?}`");
        log::info!("name is `{name}`");
        if let None = command {
            println!(
                "Please use a valid command `{:?}` or `{:?}`",
                Command::Add,
                Command::Remove
            );
            continue;
        }
        break;
    }
}

#[cfg(test)]
mod tests {
    use super::{parse, Action, Command};

    #[test]
    fn parse_add_command() {
        let Action { command, .. } = parse(String::from("Add Wendy to Pirates"));
        assert!(matches!(command, Some(Command::Add)));
        let Action { command, .. } = parse(String::from("add Wendy to Pirates"));
        assert!(matches!(command, Some(Command::Add)));
        let Action { command, .. } = parse(String::from("ADD Wendy to Pirates"));
        assert!(matches!(command, Some(Command::Add)));
    }

    #[test]
    fn parse_remove_command() {
        let Action { command, .. } = parse(String::from("Remove Wendy from Pirates"));
        assert!(matches!(command, Some(Command::Remove)));
        let Action { command, .. } = parse(String::from("remove Wendy from Pirates"));
        assert!(matches!(command, Some(Command::Remove)));
        let Action { command, .. } = parse(String::from("REMOVE Wendy from Pirates"));
        assert!(matches!(command, Some(Command::Remove)));
    }

    #[test]
    fn parse_command_as_none() {
        let Action { command, .. } = parse(String::from("Move Wendy into Pirates"));
        assert!(matches!(command, None));
        let Action { command, .. } = parse(String::from("Affect Wendy to Pirates"));
        assert!(matches!(command, None));
        let Action { command, .. } = parse(String::from("Fire Wendy from Pirates"));
        assert!(matches!(command, None));
    }
}
