use department_interface::{print, retrieve};
use std::collections::HashMap;
use std::io;

fn main() {
    let mut department_map: HashMap<String, &str> = HashMap::from([
        (String::from("Hans"), "sales"),
        (String::from("Peter"), "hr"),
        (String::from("Melissa"), "marketing"),
        (String::from("Samantha"), "hr"),
    ]);
    let department_options: [&str; 3] = ["sales", "marketing", "hr"];

    let mut employee_name = String::new();
    let mut department_input = String::new();

    println!("Please enter the employee name");

    io::stdin()
        .read_line(&mut employee_name)
        .expect("Failed to read line");

    employee_name = employee_name.trim().to_string();

    let department_selection: usize = retrieve::get_department_input(&mut department_input);

    department_map
        .entry(employee_name)
        .insert_entry(department_options[department_selection]);

    let list_option: u8 = retrieve::get_list_option();

    match list_option {
        1 => {
            let department_selection = retrieve::get_department_input(&mut department_input);
            print::print_people_in_department(
                &department_map,
                department_options[department_selection],
            )
        }
        2 => print::print_all_people_in_company(&department_map),
        _ => (),
    }
}
