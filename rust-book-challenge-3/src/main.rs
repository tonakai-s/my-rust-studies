//Using a hash map and vectors, create a text interface
//to allow a user to add employee names to a department
//in a company. For example, “Add Sally to Engineering”
//or “Add Amir to Sales.” Then let the user retrieve
//a list of all people in a department or all people
//in the company by department, sorted alphabetically.
use std::collections::HashMap;
use std::io;

fn main() {
    let mut company_employees: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        println!("What do you need? Add or List? [Type 'Quit' to exit safely]");

        let mut solicitation = String::new();

        io::stdin()
            .read_line(&mut solicitation)
            .unwrap();

        match solicitation.as_str().trim() {
            "Add" => add_employee(
                &mut company_employees,
            ),
            "List" => list_employees(&company_employees),
            "Quit" => {
                println!("Good bye lovely people! (ღ˘⌣˘ღ)");
                break;
            }
            _ => {
                println!("Invalid operation! ┻━┻ ︵ヽ(`Д´)ﾉ︵ ┻━┻");
                break;
            },
        }
    }
}

fn add_employee(
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

fn list_employees(
    map_of_employees: &HashMap<String, Vec<String>>
) {
    println!("Here's the map of the company:");
    println!("{:#?}", map_of_employees);
}