use crate::company::{Department, Employee};

#[derive(Debug)]
pub enum Command {
    Add,
    List,
    Remove,
}

pub struct Action {
    pub command: Command,
    pub employee: Option<Employee>,
    pub department: Department,
}

pub fn parse<'a>(input: String) -> Result<Action, String> {
    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.len() != 2 && parts.len() != 4 {
        return Err("Please provide a valid action!".to_string());
    }

    let command = match parts.get(0).unwrap().to_lowercase() {
        _c if _c == String::from("add") => Command::Add,
        _c if _c == String::from("remove") => Command::Remove,
        _c if _c == String::from("list") => Command::List,
        _ => {
            return Err(format!(
                "Please use a valid command `{:?}`, `{:?}` or `{:?}`",
                Command::Add,
                Command::Remove,
                Command::List
            ))
        }
    };

    let employee: Option<Employee>;
    let department: Department;
    match command {
        Command::Add | Command::Remove => {
            employee = Some(parts.get(1).unwrap().to_string());
            department = parts.get(3).unwrap().to_string();
        }
        Command::List => {
            employee = None;
            department = parts.get(1).unwrap().to_string();
        }
    }
    Ok(Action {
        command,
        department,
        employee,
    })
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
    fn parse_list_command() {
        let Action { command, .. } = parse(String::from("List X-Men")).unwrap();
        assert!(matches!(command, Command::List));
        let Action { command, .. } = parse(String::from("list Avengers")).unwrap();
        assert!(matches!(command, Command::List));
        let Action { command, .. } = parse(String::from("LIST Pirates")).unwrap();
        assert!(matches!(command, Command::List));
    }

    #[test]
    #[should_panic(expected = "Please use a valid command `Add`, `Remove` or `List`")]
    fn parse_command_unknown_move() {
        parse(String::from("Move Wendy into Pirates")).unwrap();
    }

    #[test]
    #[should_panic(expected = "Please use a valid command `Add`, `Remove` or `List`")]
    fn parse_command_unknown_affect() {
        parse(String::from("Affect Wendy to Pirates")).unwrap();
    }

    #[test]
    #[should_panic(expected = "Please use a valid command `Add`, `Remove` or `List`")]
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
    fn parse_action_too_few_words_three() {
        parse("wrong command indeed".to_string()).unwrap();
    }

    #[test]
    #[should_panic(expected = "Please provide a valid action!")]
    fn parse_action_too_many_words() {
        parse("wrong command again and again".to_string()).unwrap();
    }

    #[test]
    fn parse_add_action_employee() {
        let Action { employee, .. } = parse("Add Wendy to Pirates".to_string()).unwrap();
        assert_eq!(employee, Some(String::from("Wendy")));
    }

    #[test]
    fn parse_remove_action_employee() {
        let Action { employee, .. } = parse("Remove Vision From Avengers".to_string()).unwrap();
        assert_eq!(employee, Some(String::from("Vision")));
    }

    #[test]
    fn parse_list_action_employee() {
        let Action { employee, .. } = parse("List Pirates".to_string()).unwrap();
        assert_eq!(employee, None);
    }

    #[test]
    fn parse_add_action_department() {
        let Action { department, .. } = parse("Add Robert to Cure".to_string()).unwrap();
        assert_eq!(department, "Cure");
    }

    #[test]
    fn parse_remove_action_department() {
        let Action { department, .. } = parse("Remove Curt from Nirvana".to_string()).unwrap();
        assert_eq!(department, "Nirvana");
    }

    #[test]
    fn parse_list_action_department() {
        let Action { department, .. } = parse("List Beatles".to_string()).unwrap();
        assert_eq!(department, "Beatles");
    }
}
