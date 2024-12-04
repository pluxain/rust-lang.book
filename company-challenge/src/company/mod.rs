use std::collections::HashMap;

pub type Company = HashMap<String, Vec<String>>;
pub type Employee = String;
pub type Department = String;

pub fn add_to(company: &mut Company, department: Department, employee: Employee) {
    let d = company.entry(department.clone()).or_insert(vec![]);
    if !d.contains(&employee) {
        let msg = format!("Added `{}` to `{}`, {:?}", employee, department, d);
        d.push(employee);
        log::info!("{msg}");
    } else {
        log::warn!("`{}` is already in `{}`, {:?}", employee, department, d);
    }
}

pub fn remove_from(company: &mut Company, department: Department, employee: Employee) {
    let opt = company.get_mut(&department);
    match opt {
        Some(d) => {
            d.iter().position(|e| *e == employee).map(|index| {
                d.remove(index);
                log::info!("Removed `{}` from `{}`, {:?}", employee, department, d);
            });
        }
        None => {
            log::warn!(
                "There is no `{}` department int the company, {:?}",
                department,
                company
            );
        }
    };
}

// TODO: find a better name
fn get_department_text(employees: &mut Vec<Employee>, department: &Department) -> String {
    format!("{} department has: {}", department, employees.join(", "))
}

pub fn get_department_sorted_text(company: &mut Company, department: &Department) -> String {
    let opt = company.get_mut(department);
    match opt {
        Some(d) => {
            d.sort();
            get_department_text(d, department)
        }
        None => format!(
            "There is no {} department in the company, sorry!",
            department
        ),
    }
}

pub fn get_company_text(company: &mut Company) -> String {
    if company.is_empty() {
        String::from("The company is empty!")
    } else {
        let mut department_texts: Vec<String> = vec![String::from("The company is composed of:")];
        for (department, employees) in company {
            employees.sort();
            department_texts.push(format!(
                "\t{}\n\t\t{}",
                department,
                get_department_text(employees, department)
            ));
        }

        department_texts.join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_employee_and_department_to_empty_company() {
        let expected = Company::from([(String::from("Avengers"), vec![String::from("Thor")])]);
        let mut company = Company::new();
        add_to(&mut company, String::from("Avengers"), String::from("Thor"));
        assert_eq!(company, expected);
    }

    #[test]
    fn add_employee_to_department_to_non_empty_company() {
        let expected = Company::from([
            (String::from("Avengers"), vec![String::from("Thor")]),
            (String::from("X-Men"), vec![String::from("Professor X")]),
        ]);
        let mut company =
            Company::from([(String::from("X-Men"), vec![String::from("Professor X")])]);
        add_to(&mut company, String::from("Avengers"), String::from("Thor"));
        assert_eq!(company, expected);
    }

    #[test]
    fn add_employee_to_existing_department() {
        let expected = Company::from([(
            String::from("Avengers"),
            vec![String::from("Thor"), String::from("Iron Man")],
        )]);
        let mut company = Company::from([(String::from("Avengers"), vec![String::from("Thor")])]);
        add_to(
            &mut company,
            String::from("Avengers"),
            String::from("Iron Man"),
        );
        assert_eq!(company, expected);
    }

    #[test]
    fn add_employee_once_to_department() {
        let expected = Company::from([(String::from("Avengers"), vec![String::from("Thor")])]);
        let mut company = Company::from([(String::from("Avengers"), vec![String::from("Thor")])]);
        add_to(&mut company, String::from("Avengers"), String::from("Thor"));
        assert_eq!(company, expected);
    }

    #[test]
    fn remove_last_employee_from_department() {
        let expected = Company::from([(String::from("Pirates"), vec![])]);
        let mut company = Company::from([(String::from("Pirates"), vec![String::from("Peter")])]);
        remove_from(&mut company, String::from("Pirates"), String::from("Peter"));
        assert_eq!(company, expected);
    }

    #[test]
    fn remove_one_employee_from_department() {
        let expected = Company::from([(
            String::from("Pirates"),
            vec![String::from("Peter"), String::from("Wendy")],
        )]);
        let mut company = Company::from([(
            String::from("Pirates"),
            vec![
                String::from("Peter"),
                String::from("Tinker Bell"),
                String::from("Wendy"),
            ],
        )]);
        remove_from(
            &mut company,
            String::from("Pirates"),
            String::from("Tinker Bell"),
        );
        assert_eq!(company, expected);
    }

    #[test]
    fn does_not_remove_unknown_employee_from_department() {
        let expected = Company::from([(String::from("Avengers"), vec![String::from("Thor")])]);
        let mut company = Company::from([(String::from("Avengers"), vec![String::from("Thor")])]);
        remove_from(
            &mut company,
            String::from("Avengers"),
            String::from("Iron Man"),
        );
        assert_eq!(company, expected);
    }

    #[test]
    fn does_not_remove_employee_from_unknown_department() {
        let expected = Company::from([(String::from("Avengers"), vec![String::from("Thor")])]);
        let mut company = Company::from([(String::from("Avengers"), vec![String::from("Thor")])]);
        remove_from(&mut company, String::from("Pirates"), String::from("Thor"));
        assert_eq!(company, expected);
    }

    #[test]
    fn does_not_remove_employee_from_department_when_employee_not_in() {
        let expected = Company::from([
            (String::from("Avengers"), vec![String::from("Thor")]),
            (String::from("Pirates"), vec![String::from("Wendy")]),
        ]);
        let mut company = Company::from([
            (String::from("Avengers"), vec![String::from("Thor")]),
            (String::from("Pirates"), vec![String::from("Wendy")]),
        ]);
        remove_from(&mut company, String::from("Pirates"), String::from("Thor"));
        remove_from(
            &mut company,
            String::from("Avengers"),
            String::from("Wendy"),
        );
        assert_eq!(company, expected);
    }

    #[test]
    fn get_single_list() {
        let mut company = Company::from([(String::from("CLanguages"), vec![String::from("C")])]);
        assert_eq!(
            get_department_sorted_text(&mut company, &String::from("CLanguages")),
            "CLanguages department has: C"
        );
    }

    #[test]
    fn get_multiple_list() {
        let mut company = Company::from([(
            String::from("CLanguages"),
            vec![String::from("C"), String::from("Java"), String::from("C++")],
        )]);
        assert_eq!(
            get_department_sorted_text(&mut company, &String::from("CLanguages")),
            "CLanguages department has: C, C++, Java"
        );
    }

    #[test]
    fn get_no_department() {
        let mut company = Company::new();
        assert_eq!(
            get_department_sorted_text(&mut company, &String::from("CLanguages")),
            "There is no CLanguages department in the company, sorry!"
        );
    }

    #[test]
    fn get_empty_company() {
        let mut company = Company::new();
        assert_eq!(get_company_text(&mut company), "The company is empty!");
    }

    #[test]
    fn get_one_department_company() {
        let mut company = Company::from([(
            String::from("X-Men"),
            vec![
                String::from("ProfessorX"),
                String::from("Cyclop"),
                String::from("Diablo"),
            ],
        )]);
        assert_eq!(
            get_company_text(&mut company),
            "The company is composed of:\n\tX-Men\n\t\tX-Men department has: Cyclop, Diablo, ProfessorX"
        );
    }
}
