//Using a hash map and vectors, create a text interface
//to allow a user to add employee names to a department
//in a company. For example, “Add Sally to Engineering”
//or “Add Amir to Sales.” Then let the user retrieve
//a list of all people in a department or all people
//in the company by department, sorted alphabetically.
use std::{io, collections::HashMap};

mod employee_actions;

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
            "Add" => employee_actions::add::employee(
                &mut company_employees,
            ),
            "List" => employee_actions::list::employees(&company_employees),
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