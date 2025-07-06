// Using a hash map and vectors, create a text interface to allow a user to add employee names
// to a department in a company; for example, "Add Sally to Engineering" or "Add Amir to Sales." Then
// let the user retrieve a list of all people in a department or all people in the company by department,
// sorted alphabetically.
use std::collections::HashMap;
use std::io;

struct Company {
    departments: HashMap<String, Vec<String>>,
}

impl Company {
    fn new() -> Self {
        Company {
            departments: HashMap::new(),
        }
    }

    fn add_employee(&mut self, employee: String, department: String) {
        self.departments
            .entry(department)
            .or_insert(Vec::new())
            .push(employee);
    }

    fn get_employees(&self, department: &str) -> Option<Vec<String>> {
        self.departments.get(department).map(|v| {
            let mut sorted = v.clone();
            sorted.sort();
            sorted
        })
    }

    fn get_all_employees(&self) -> HashMap<String, Vec<String>> {
        let mut all = HashMap::new();
        for (dept, employees) in &self.departments {
            let mut sorted = employees.clone();
            sorted.sort();
            all.insert(dept.clone(), sorted);
        }
        all
    }
}

//provide a text interface to
// 1. allow a user to add employee names to a department in a company or
// 2. or retrieve a list of all people in a department or
// 3. all people in the company by department, sorted alphabetically.

pub fn input_command() {
    let mut company = Company::new();
    loop {
        println!("\n1. Add employee to department");
        println!("2. Retrieve all employees in a department");
        println!("3. Retrieve all employees in the company by department, sorted alphabetically");
        println!("4. Exit");
        println!("Enter your choice:");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        let choice = choice.trim();

        match choice {
            "1" => add_employee(&mut company),
            "2" => retrieve_employees_in_department(&company),
            "3" => retrieve_all_employees_by_department(&company),
            "4" => {
                println!("Exiting.");
                break;
            }
            _ => println!("Invalid choice"),
        }
    }
}

fn add_employee(company: &mut Company) {
    println!("Enter employee name:");
    let mut employee = String::new();
    io::stdin()
        .read_line(&mut employee)
        .expect("Failed to read line");
    let employee = employee.trim().to_string();

    println!("Enter department name:");
    let mut department = String::new();
    io::stdin()
        .read_line(&mut department)
        .expect("Failed to read line");
    let department = department.trim().to_string();

    company.add_employee(employee, department);
    println!("Employee added.");
}

fn retrieve_employees_in_department(company: &Company) {
    println!("Enter department name:");
    let mut department = String::new();
    io::stdin()
        .read_line(&mut department)
        .expect("Failed to read line");
    let department = department.trim();

    match company.get_employees(department) {
        Some(employees) if !employees.is_empty() => {
            println!("Employees in {}: {:?}", department, employees);
        }
        _ => println!("No employees found in this department."),
    }
}

fn retrieve_all_employees_by_department(company: &Company) {
    let all = company.get_all_employees();
    if all.is_empty() {
        println!("No employees in the company.");
        return;
    }
    // Collect and sort the department names (keys)
    let mut departments: Vec<_> = all.keys().collect();
    departments.sort();

    for dept in departments {
        if let Some(employees) = all.get(dept) {
            println!("Department: {} => {:?}", dept, employees);
        }
    }
}
