use guessing_game::guess;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number: u32 = rand::rng().random_range(1..=100);

    loop {
        let mut guess = String::new();

        guess::read_input(&mut guess);

        if guess::compare_guess(&mut guess, secret_number) {
            break;
        }
    }
}
