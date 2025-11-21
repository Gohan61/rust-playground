use std::collections::HashMap;
use std::io;

fn main() {
    let mut department_map: HashMap<String, &str> = HashMap::from([
        (String::from("Hans"), "sales"),
        (String::from("Peter"), "hr"),
        (String::from("Melissa"), "marketing"),
    ]);
    let department_options: [&str; 3] = ["sales", "marketing", "hr"];

    let mut employee_name = String::new();
    let mut department_input = String::new();

    loop {
        employee_name.clear();
        println!("Please enter the employee name");

        io::stdin()
            .read_line(&mut employee_name)
            .expect("Failed to read line");

        employee_name = employee_name.trim().to_string();
        break;
    }

    let department_selection: usize = get_department_input(&mut department_input);

    department_map
        .entry(employee_name)
        .or_insert(department_options[department_selection]);

    // println!("{:?}", department_map);

    let list_option: u8 = get_list_option();
}

fn get_list_option() -> u8 {
    let mut retrieve_list_option = String::new();

    loop {
        retrieve_list_option.clear();
        println!("\nPress 1: to retrieve list of all people in a department");
        println!("Press 2: to retrieve list of all people in the company");

        io::stdin()
            .read_line(&mut retrieve_list_option)
            .expect("Failed to read line");

        let retrieve_list_option: u8 = match retrieve_list_option.trim().parse() {
            Ok(1) => 1,
            Ok(2) => 2,
            _ => {
                println!("\nInvalid selection");
                continue;
            }
        };

        break retrieve_list_option;
    }
}

fn get_department_input(department_input: &mut String) -> usize {
    loop {
        department_input.clear();
        println!("Please select a department");

        io::stdin()
            .read_line(department_input)
            .expect("Failed to read line");

        let department_selection = match department_input.trim().parse() {
            Ok(1) => 0,
            Ok(2) => 1,
            Ok(3) => 2,
            _ => {
                println!("\nInvalid selection");
                continue;
            }
        };
        break department_selection;
    }
}
