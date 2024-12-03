// Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company;
// for example, “Add Sally to Engineering” or “Add Amir to Sales.”
// Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.

use crate::action::{parse, Action, Command};
use crate::prompt::prompt;
use company::{add_to, remove_from, Company};
use log;
use log4rs;

pub mod action;
pub mod company;
pub mod prompt;

fn main() {
    log4rs::init_file("config/logger.yaml", Default::default()).unwrap();
    log::info!("challenge: company");
    let mut company = Company::new();

    loop {
        let input = prompt(format!(
            "What do you want to do (ex: `{:?} Robert to DevOps` or `{:?} Wendy from Pirates`), Ctrl + C to exit the programm",
            Command::Add,
            Command::Remove
        ))
        .unwrap();
        log::info!("input is `{input}`");

        let result = parse(input);
        let Action {
            command,
            department,
            employee,
        } = match result {
            Ok(action) => action,
            Err(error) => {
                println!("{error}");
                continue;
            }
        };
        log::info!("command is `{command:?}`");
        log::info!("employee is `{employee}`");
        log::info!("department is `{department}`");

        match command {
            Command::Add => add_to(&mut company, department, employee),
            Command::Remove => remove_from(&mut company, department, employee),
            _ => todo!(),
        }
        log::debug!("{:?}", company);
        continue;
    }
}

#[cfg(test)]
mod tests {}
