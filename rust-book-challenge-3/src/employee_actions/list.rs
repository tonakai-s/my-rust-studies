use std::{io, collections::HashMap};

pub fn employees(
    map_of_employees: &HashMap<String, Vec<String>>
) {
    println!("\nType 'All' to list all company, or type the Department you whant to look: ");
    let mut list_type = String::new();

    io::stdin()
        .read_line(&mut list_type)
        .unwrap();
    
    list_type = list_type.trim().to_string();

    match list_type.as_str() {
        "All" => list_all_company(&map_of_employees),
        _ => list_by_department(&list_type, &map_of_employees),
    }
}

fn list_by_department(department: &String, company_map: &HashMap<String, Vec<String>>){
    let department_employees: Option<&Vec<String>> = company_map.get(department);
    match department_employees {
        Some(employees) => {
            let mut employees = employees.clone();
            employees.sort();
            
            println!("The employees of the department {department}:");
            for employee in employees {
                println!("• {}\n", employee);
            }
        },
        None => println!("This department not exist!")
    }
}

fn list_all_company(company_map: &HashMap<String, Vec<String>>) {
    println!("\nHere is the map of the company!");
    println!("Printing on pattern [Department ➳ Employees]\n");

    for(key, value) in company_map {
        let mut employees = value.clone();
        employees.sort();

        println!("{key} ➳  {}\n", employees.join(" | "));
    }
}