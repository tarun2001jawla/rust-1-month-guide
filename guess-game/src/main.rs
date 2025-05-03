use rand::Rng;
use std::io;

fn main() {
    println!("Welcome to the Guessing Game.");
    println!("I'm thinking of a number between 1 and 100....");
    println!("You have 5 attempts to guess the correct number!");

    let secret_number_new = rand::thread_rng().gen_range(1..=100);
    let mut attempts = 0;

    while attempts < 5 {
        println!("\nAttempt {}/5: Enter your guess:", attempts + 1);

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number");
                continue;
            }
        };
        attempts += 1;

        if guess == secret_number_new {
            println!(
                "Congratulations! You guessed it right, the number was: {}",
                secret_number_new
            );
            return;
        } else if guess < secret_number_new {
            println!("Too low!");
        } else {
            println!("Too high!");
        }
    }
    println!(
        "\n❌ You've used all your attempts. The number was {}.",
        secret_number_new
    );
}
