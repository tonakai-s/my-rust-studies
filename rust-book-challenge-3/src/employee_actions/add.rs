use std::{io, collections::HashMap};

pub fn employee(
    company_map: &mut HashMap<String, Vec<String>>,
) {
    let mut employee_name = String::new();

    println!("\nWich the first name of the employee?");
    io::stdin()
        .read_line(&mut employee_name)
        .unwrap();

    let mut department = String::new();
    println!("\nAdd him into wich department?");
    io::stdin()
        .read_line(&mut department)
        .unwrap();

    let current_employees = company_map.entry(department.trim().to_string()).or_insert(Vec::new());
    current_employees.push(employee_name.trim().to_string());

    println!("✿ Employee added successfully! ✿");
    println!("---------------------------------\n");
}