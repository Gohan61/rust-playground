use std::collections::HashMap;
use std::io;

fn main() {
    let mut department_map: HashMap<&str, String> = HashMap::new();
    let department_options: [&str; 3] = ["sales", "marketing", "hr"];

    let mut employee_name = String::new();

    loop {
        println!("Please enter the employee name");

        io::stdin()
            .read_line(&mut employee_name)
            .expect("Failed to read line");
    }
}
