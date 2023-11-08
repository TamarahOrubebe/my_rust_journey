use std::collections::HashMap;
use std::io;

fn main () {
   
    

    let mut company: HashMap<String, Vec<String>> = HashMap::new();
    

    loop {
        println!("Enter a command e.g Add Sally to Engineering or Add Amir to Sales, or List all company employees, or List
        employees in department_name");
        
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input).expect("failed to read line");

        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        let parts: Vec<&str> = input.split(" ").collect();

        if parts.len() < 4 {
            println!("Pls enter a valid commad e.g Add Name to Department, or List employees in Department
            , or List all company employees");
            continue;
        }

        let name = parts[1];
        let department = parts[3];

        match parts[0] {
            "Add" => company.entry(department.to_string()).or_insert(Vec::new()).push(name.to_string()),
            "List" => {
                if department == "employees" {
                    list_all_employees(&company);
                } else {
                    list_employees_per_department(&company, department);
                }
            },
            _ => println!("Please enter a valid comman using Add or List")

        }



    }


    fn list_all_employees(company: &HashMap<String, Vec<String>>) {
        let mut employee_list: Vec<String> = Vec::new();

        for department_employees in company.values() {
            employee_list.extend(department_employees.iter().cloned());
        }
        employee_list.sort();

        println!("The names of all the company employees are {:?}", employee_list);
    }

    fn list_employees_per_department(company: &HashMap<String, Vec<String>>, department: &str ) {
        
        if let Some(employees) = company.get(department) {

            let mut department_employees = employees.clone();

            department_employees.sort();
            println!("The names of employees in the {} department are: {:?}", department, department_employees);
        } else {
            println!("Department {} not found", department);
        }

    }

    
}





    
// Using a hash map and vectors, create a text interface to allow a user to add employee names to a department
// in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.”
// Then let the user retrieve a list of all people in a department or all people in the company by department, 
// sorted alphabetically.
