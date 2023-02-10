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

        solicitation = solicitation.trim().to_string();

        match solicitation.as_str() {
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
    println!("Type 'All' to list all company, or type the Department you whant to look: ");
    let mut list_type = String::new();

    io::stdin()
        .read_line(&mut list_type)
        .unwrap();
    
    list_type = list_type.trim().to_string();

    match list_type.as_str() {
        "All" => list_all_company(&map_of_employees),
        _ => {
            println!("{:?}", list_type);
            list_by_department(&list_type, &map_of_employees)
        },
    }
}

fn list_by_department(department: &String, company_map: &HashMap<String, Vec<String>>){
    let department_employees: Option<&Vec<String>> = company_map.get(department);
    match department_employees {
        Some(employees) => {
            let mut employees = employees.clone();
            employees.sort();

            println!("The employees of the department {department}:");
            println!("{}", employees.join("\r\n"));
        },
        None => println!("This department not exist!")
    }
}

fn list_all_company(company_map: &HashMap<String, Vec<String>>) {
    println!("Here is the map of the company!");
    println!("Printing on pattern [Department => Employees]");

    for(key, value) in company_map {
        let mut employees = value.clone();
        employees.sort();

        println!("{key} => {}", employees.join(" | "));
    }
}