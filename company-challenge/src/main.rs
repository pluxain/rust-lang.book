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
    command: Command,
    name: String,
    department: String,
}

fn prompt(p: String) -> Result<String, io::Error> {
    log::info!("Prompting");
    let mut input = String::new();
    println!("{p}");
    match stdin().read_line(&mut input) {
        Ok(_) => Ok(input.trim().to_string()),
        Err(err) => Err(err),
    }
}

fn parse<'a>(input: String) -> Result<Action, String> {
    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.len() < 4 {
        return Err("Please provide a valid action!".to_string());
    }
    let command = match parts.get(0).unwrap().to_lowercase() {
        _c if _c == String::from("add") => Command::Add,
        _c if _c == String::from("remove") => Command::Remove,
        _ => {
            return Err(format!(
                "Please use a valid command `{:?}` or `{:?}`",
                Command::Add,
                Command::Remove
            ))
        }
    };
    let name = parts.get(1).unwrap().to_string();
    let department = parts.get(3).unwrap().to_string();
    Ok(Action {
        command,
        department,
        name,
    })
}

fn main() {
    log4rs::init_file("config/logger.yaml", Default::default()).unwrap();
    log::info!("challenge: company");

    loop {
        let input = prompt(format!(
            "What do you want to do (ex: `{:?} Robert to DevOps` or `{:?} Wendy from Pirates`)",
            Command::Add,
            Command::Remove
        ))
        .unwrap();
        log::info!("input is `{input}`");

        let result = parse(input);
        let Action {
            command,
            department,
            name,
        } = match result {
            Ok(action) => action,
            Err(error) => {
                println!("{error}");
                continue;
            }
        };
        log::info!("command is `{command:?}`");
        log::info!("name is `{name}`");
        log::info!("department is `{department}`");
        break;
    }
}

#[cfg(test)]
mod tests {
    use super::{parse, Action, Command};

    #[test]
    fn parse_add_command() {
        let Action { command, .. } = parse(String::from("Add Wendy to Pirates")).unwrap();
        assert!(matches!(command, Command::Add));
        let Action { command, .. } = parse(String::from("add Wendy to Pirates")).unwrap();
        assert!(matches!(command, Command::Add));
        let Action { command, .. } = parse(String::from("ADD Wendy to Pirates")).unwrap();
        assert!(matches!(command, Command::Add));
    }

    #[test]
    fn parse_remove_command() {
        let Action { command, .. } = parse(String::from("Remove Wendy from Pirates")).unwrap();
        assert!(matches!(command, Command::Remove));
        let Action { command, .. } = parse(String::from("remove Wendy from Pirates")).unwrap();
        assert!(matches!(command, Command::Remove));
        let Action { command, .. } = parse(String::from("REMOVE Wendy from Pirates")).unwrap();
        assert!(matches!(command, Command::Remove));
    }

    #[test]
    #[should_panic(expected = "Please use a valid command `Add` or `Remove`")]
    fn parse_command_unknown_move() {
        parse(String::from("Move Wendy into Pirates")).unwrap();
    }

    #[test]
    #[should_panic(expected = "Please use a valid command `Add` or `Remove`")]
    fn parse_command_unknown_affect() {
        parse(String::from("Affect Wendy to Pirates")).unwrap();
    }

    #[test]
    #[should_panic(expected = "Please use a valid command `Add` or `Remove`")]
    fn parse_command_unknown_fire() {
        parse(String::from("Fire Wendy from Pirates")).unwrap();
    }

    #[test]
    #[should_panic(expected = "Please provide a valid action!")]
    fn parse_action_too_few_words_one() {
        parse("wrong".to_string()).unwrap();
    }

    #[test]
    #[should_panic(expected = "Please provide a valid action!")]
    fn parse_action_too_few_words_two() {
        parse("wrong command".to_string()).unwrap();
    }

    #[test]
    #[should_panic(expected = "Please provide a valid action!")]
    fn parse_action_too_few_words_three() {
        parse("wrong command indeed".to_string()).unwrap();
    }

    #[test]
    fn parse_action_name() {
        let Action { name, .. } = parse("Add Wendy to Pirates".to_string()).unwrap();
        assert_eq!(name, "Wendy");
    }

    #[test]
    fn parse_action_department() {
        let Action { department, .. } = parse("Add Wendy to Pirates".to_string()).unwrap();
        assert_eq!(department, "Pirates");
    }
}
