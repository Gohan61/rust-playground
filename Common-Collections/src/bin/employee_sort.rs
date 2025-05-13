use std::{
    collections::HashMap,
    io::{self, Write},
};

fn main() {
    let mut employee_name = String::new();
    let mut department = String::new();
    let mut company_db = HashMap::from([
        (String::from("Dwight"), String::from("Sales")),
        (String::from("Toby"), String::from("HR")),
        (String::from("Kate"), String::from("Customer Service")),
    ]);

    loop {
        println!("Enter employee name: ");
        io::stdout().flush().unwrap_or_default();
        match io::stdin().read_line(&mut employee_name) {
            Ok(_) if employee_name.trim().is_empty() => println!("\nDeparment cannot be empty"),
            Ok(_) if !employee_name.is_ascii() => println!("\nOnly ascii characters are allowed"),
            Ok(_) => {
                employee_name = employee_name.trim().to_string();
                break;
            }
            Err(err) => println!("\nCould not parse input: {}", err),
        }
    }

    loop {
        println!("Enter department: ");
        io::stdout().flush().unwrap_or_default();
        match io::stdin().read_line(&mut department) {
            Ok(_) if department.trim().is_empty() => println!("\nDeparment cannot be empty"),
            Ok(_) if !department.is_ascii() => println!("\nOnly ascii characters are allowed"),
            Ok(_) => {
                department = department.trim().to_string();
                break;
            }
            Err(err) => println!("\nCould not parse input: {}", err),
        }
    }

    company_db.entry(employee_name).or_insert(department);

    let mut sort_selection = String::new();
    println!("\nEnter 1 to sort by employees.\nEnter 2 to sort by deparment");
    io::stdout().flush().unwrap_or_default();
    loop {
        io::stdin()
            .read_line(&mut sort_selection)
            .unwrap_or_default();

        let sort_selection_u8 = sort_selection.trim().parse::<u8>();

        match sort_selection_u8 {
            Ok(1) => break,
            Ok(2) => break,
            _ => println!("\nPlease enter a valid entry"),
        }
    }

    sort_selection = sort_selection.trim().to_string();
    println!("---");

    if sort_selection == "1" {
        let mut sorted_db: Vec<(&String, &String)> = company_db.iter().collect();
        sorted_db.sort_by(|a, b| a.0.cmp(b.0));

        for (employee, deparment) in sorted_db {
            println!("{}: {}", employee, deparment);
        }
    } else if sort_selection == "2" {
        let mut sorted_db: Vec<(&String, &String)> = company_db.iter().collect();
        sorted_db.sort_by(|a, b| a.1.cmp(b.1));

        for (employee, deparment) in sorted_db {
            println!("{}: {}", deparment, employee);
        }
    }
}
