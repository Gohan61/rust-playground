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

    employee_name.clear();
    println!("Please enter the employee name");

    io::stdin()
        .read_line(&mut employee_name)
        .expect("Failed to read line");

    employee_name = employee_name.trim().to_string();

    let department_selection: usize = get_department_input(&mut department_input);

    department_map
        .entry(employee_name)
        .insert_entry(department_options[department_selection]);

    let list_option: u8 = get_list_option();

    match list_option {
        1 => {
            let department_selection = get_department_input(&mut department_input);
            print_people_in_department(&department_map, department_options[department_selection])
        }
        2 => print_all_people_in_company(&department_map),
        _ => (),
    }
}

fn print_all_people_in_company(department_map: &HashMap<String, &str>) {
    print!("\n");
    let mut people_list: Vec<_> = department_map
        .iter()
        .map(|(k, v)| (k.as_str(), *v))
        .collect();

    people_list.sort_by(|(k1, _), (k2, _)| k1.cmp(k2));
    println!("{:?}", people_list);
}

fn print_people_in_department(department_map: &HashMap<String, &str>, department: &str) {
    print!("\n");
    department_map
        .iter()
        .filter(|(_, v)| **v == department)
        .for_each(|(k, v)| println!("{}: {}", k, v));
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
        println!("\nPlease select a department");
        println!("Press 1: 'sales'");
        println!("Press 2: 'marketing'");
        println!("Press 3: 'hr'");

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
