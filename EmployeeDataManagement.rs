use std::fmt::Debug;

#[derive(Debug)]
struct Employee <'a>{
    employee_name: &'a str,
    employee_id: i32,
    email: &'a str,
    age: i32,
    phone_number: &'a str,
}

enum SearchType {
    ById(i32),
    ByAge(i32),
}

impl<'a> Employee <'_> {
    fn find_employee(&self, search_type: SearchType) -> bool {
        match search_type {
            SearchType::ById(id) => self.employee_id == id,
            SearchType::ByAge(age) => self.age == age,
        }
    }
}

fn main() {
    // Simulated array of employees
    let employees = [
        Employee {
            employee_name: "Warden",
            employee_id: 1,
            email: "warden@gmail.com",
            age: 30,
            phone_number: "123-456-7890",
        },
        Employee {
            employee_name: "Smith",
            employee_id: 2,
            email: "smith@gmail.com",
            age: 25,
            phone_number: "987-654-3210",
        },
        // Add more employees here
    ];

    // Search by employee ID
    let search_id = 2;
    let result = employees.iter().find(|&employee| employee.find_employee(SearchType::ById(search_id)));

    match result {
        Some(employee) => {
            println!("Employee found: {} (ID: {:?})", employee.employee_name, employee.employee_id);
        }
        None => {
            println!("Employee not found.");
        }
    }

    // Search by age
    let search_age = 30;
    let employees_with_same_age: Vec<_> = employees.iter().filter(|&employee| employee.find_employee(SearchType::ByAge(search_age))).collect();

    if employees_with_same_age.is_empty() {
        println!("No employees found with age {:?}.", search_age);
    } else {
        println!("Employees with age {}: {:?}", search_age, employees_with_same_age);
    }
}