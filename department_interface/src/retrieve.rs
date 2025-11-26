use std::io;

pub fn get_list_option() -> u8 {
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

pub fn get_department_input(department_input: &mut String) -> usize {
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
