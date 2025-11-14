use core::f64;
use std::io;
use std::num::ParseIntError;
use std::str::FromStr;

enum Options {
    One = 1,
    Two = 2,
}

impl FromStr for Options {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<u8>()? {
            1 => Ok(Options::One),
            2 => Ok(Options::Two),
            _ => Err("not a valid option".parse::<u8>().unwrap_err()),
        }
    }
}

fn main() {
    let convert_option = loop {
        println!("Press 1: convert to Celsius");
        println!("Press 2: convert to Fahrenheit");

        let mut convert_option = String::new();

        io::stdin()
            .read_line(&mut convert_option)
            .expect("Failed to read line");

        match convert_option.trim().parse::<Options>() {
            Ok(option) => break option,
            Err(_) => {
                println!("Please enter a valid option");
                continue;
            }
        };
    };

    let degree_input = loop {
        match convert_option {
            Options::One => println!("\nEnter the degrees in Fahrenheit"),
            Options::Two => println!("\nEnter the degrees in Celsius"),
        }

        let mut degree_input = String::new();

        io::stdin()
            .read_line(&mut degree_input)
            .expect("No valid input");

        match degree_input.trim().parse::<f64>() {
            Ok(num) => break num,
            Err(_) => {
                println!("Please enter a valid number");
                continue;
            }
        }
    };

    match convert_option {
        Options::One => println!("\n{:.1} Celsius", to_celsius(degree_input)),
        Options::Two => println!("\n{:.1} Fahrenheit", to_fahrenheit(degree_input)),
    }
}

fn to_celsius(temp: f64) -> f64 {
    (temp - 32.0) / 1.8
}

fn to_fahrenheit(temp: f64) -> f64 {
    temp * 1.8 + 32.0
}
