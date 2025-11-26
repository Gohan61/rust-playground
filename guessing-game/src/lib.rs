pub mod guess {
    use std::cmp::Ordering;
    use std::io;

    pub fn read_input(guess: &mut String) {
        println!("Please input your guess.");

        io::stdin().read_line(guess).expect("Failed to read line");
    }

    pub fn compare_guess(input_guess: &mut String, secret_number: u32) -> bool {
        if let Ok(guess) = input_guess.trim().parse::<u32>() {
            println!("\nYou guessed: {}", guess);

            match guess.cmp(&secret_number) {
                Ordering::Less => {
                    println!("\nToo small!");
                }
                Ordering::Greater => {
                    println!("\nToo big!");
                }
                Ordering::Equal => {
                    println!("\nYou win!");
                    return true;
                }
            }
        } else {
            println!("\nPlease enter a valid number");
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::guess::compare_guess;

    #[test]
    fn too_small_guess() {
        assert!(compare_guess(&mut String::from("1"), 1))
    }

    #[test]
    fn too_big_guess() {
        assert!(!compare_guess(&mut String::from("2"), 1));
    }

    #[test]
    fn guess_right() {
        assert!(compare_guess(&mut String::from("1"), 1));
    }

    #[test]
    fn invalid_char() {
        assert!(!compare_guess(&mut String::from("a"), 1));
    }
}
