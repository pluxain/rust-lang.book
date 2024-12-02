use std::collections::HashMap;

pub type Company = HashMap<String, Vec<String>>;
pub type Employee = String;
pub type Department = String;

pub fn add_to(company: &mut Company, department: Department, employee: Employee) {
    let d = company.entry(department.clone()).or_insert(vec![]);
    if !d.contains(&employee) {
        d.push(employee.clone());
        log::info!("Added {} to {}, {:?}", employee, department, d);
    } else {
        log::warn!("{} is already in {}, {:?}", employee, department, d);
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
}
