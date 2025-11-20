use std::collections::HashMap;
use std::io;

fn main() {
    let mut department_map: HashMap<&str, String> = HashMap::new();
    let department_options: [&str; 3] = ["sales", "marketing", "hr"];

    let mut employee_name = String::new();
    let mut department_input = String::new();
    let mut department_selection: u8;

    loop {
        employee_name.clear();
        department_input.clear();
        println!("Please enter the employee name");

        io::stdin()
            .read_line(&mut employee_name)
            .expect("Failed to read line");

        println!("Please select a department");

        io::stdin()
            .read_line(&mut department_input)
            .expect("Failed to read line");

        department_selection = match department_input.trim().parse() {
            Ok(1) => 0,
            Ok(2) => 1,
            Ok(3) => 2,
            _ => {
                println!("\nInvalid selection");
                continue;
            }
        };
        break;
    }
}
