use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Welcome to the Guessing Game!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("I've picked a number between 1 and 100. Can you guess what it is?");

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("That doesn't seem like a number. Please enter a valid number:");
                continue;
            }
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too low! Try again:"),
            Ordering::Greater => println!("Too high! Try again:"),
            Ordering::Equal => {
                println!("Congratulations! You guessed the right number!");
                break;
            }
        }
    }
}
