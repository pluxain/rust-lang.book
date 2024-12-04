// Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company;
// for example, “Add Sally to Engineering” or “Add Amir to Sales.”
// Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.

use crate::action::{parse, Action, Command};
use crate::prompt::prompt;
use company::{add_to, get_company_text, get_department_sorted_text, remove_from, Company};
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
            "What do you want to do (ex: `{:?} Robert to DevOps`, `{:?} Wendy from Pirates` or `{:?} Avengers`), Ctrl + C to exit the programm",
            Command::Add,
            Command::Remove,
            Command::List
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
        log::info!("department is `{department}`");
        log::info!(
            "employee is `{}`",
            employee.clone().unwrap_or(String::from("None"))
        );

        match command {
            Command::Add => add_to(&mut company, department, employee.unwrap()),
            Command::Remove => remove_from(&mut company, department, employee.unwrap()),
            Command::List => match department.to_lowercase() {
                _c if _c == String::from("company") => {
                    println!("{}", get_company_text(&mut company))
                }

                _ => println!("{}", get_department_sorted_text(&mut company, &department)),
            },
        }
        log::debug!("{:?}", company);
        continue;
    }
}

#[cfg(test)]
mod tests {}
