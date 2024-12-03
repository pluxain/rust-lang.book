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

pub fn sort_department(company: &mut Company, department: &Department) {
    let opt = company.get_mut(department);
    match opt {
        Some(d) => {
            d.sort();
        }
        None => {
            log::warn!(
                "There is no `{}` department int the company, {:?}",
                department,
                company
            );
        }
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
    fn sort_existing_department() {
        let expected = HashMap::from([
            (
                String::from("Cure"),
                vec![String::from("Robert"), String::from("Simon")],
            ),
            (
                String::from("Beatles"),
                vec![
                    String::from("George"),
                    String::from("John"),
                    String::from("Paul"),
                    String::from("Ringo"),
                ],
            ),
            (
                String::from("RollingStones"),
                vec![
                    String::from("Charlie"),
                    String::from("Keith"),
                    String::from("Mick"),
                ],
            ),
        ]);

        let mut company = HashMap::from([
            (
                String::from("Cure"),
                vec![String::from("Robert"), String::from("Simon")],
            ),
            (
                String::from("Beatles"),
                vec![
                    String::from("Paul"),
                    String::from("Ringo"),
                    String::from("George"),
                    String::from("John"),
                ],
            ),
            (
                String::from("RollingStones"),
                vec![
                    String::from("Mick"),
                    String::from("Keith"),
                    String::from("Charlie"),
                ],
            ),
        ]);

        sort_department(&mut company, &String::from("Cure"));
        assert_eq!(
            company.get(&String::from("Cure")).unwrap().to_vec(),
            vec![String::from("Robert"), String::from("Simon"),]
        );

        sort_department(&mut company, &String::from("Beatles"));
        assert_eq!(
            company.get(&String::from("Beatles")).unwrap().to_vec(),
            vec![
                String::from("George"),
                String::from("John"),
                String::from("Paul"),
                String::from("Ringo"),
            ]
        );

        sort_department(&mut company, &String::from("RollingStones"));
        assert_eq!(
            company
                .get(&String::from("RollingStones"))
                .unwrap()
                .to_vec(),
            vec![
                String::from("Charlie"),
                String::from("Keith"),
                String::from("Mick"),
            ]
        );

        assert_eq!(company, expected);
    }

    #[test]
    fn does_not_sort_unknown_department() {
        let expected = Company::from([(
            String::from("Beatles"),
            vec![String::from("Ringo"), String::from("George")],
        )]);
        let mut company = Company::from([(
            String::from("Beatles"),
            vec![String::from("Ringo"), String::from("George")],
        )]);

        sort_department(&mut company, &String::from("RollingStones"));
        assert_eq!(company, expected);
    }
}
