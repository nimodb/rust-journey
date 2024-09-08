# 2. Programming a Guessing Game

This Rust project is a simple command-line guessing game where the player tries to guess a randomly generated number between 1 and 100.

## Code Breakdown

1. Adding Dependencies

   - First, modify the `Cargo.toml` file to include the `rand` crate as a dependency:

   ```bash
   [dependencies]
   rand = "0.8.5"
   ```

   - The `rand` crate is used to generate random numbers in the game.

2. Importing Modules

   ```rust
   use rand::Rng;
   use std::cmp::Ordering;
   use std::io;
   ```

   - `rand::Rng`: Provides random number generation functionality.
   - `std::cmp::Ordering`: Allow comparison between the guessed number and the secret number.
   - `std::io`: Handles input/output, specifically to read user input.

3. Main Function

   ```rust
   fn main() {
       println!("Welcome to the Guessing Game!");
   ```

   - The `main` function is the entry point of the program.
   - The first `println!` displays a welcome message to the user.

4. Generating a Random Number and Game Instructions

   ```rust
   let secret_number = rand::thread_rng().gen_range(1..=100);
   ```

   - `rand::thread_rang()`: Acquires a random number generator.
   - `.get_range(1..=100)`: Generates a random number between 1 and 100

   ```rust
   println!("I've picked a number between 1 and 100. Can you guess what it is?");
   ```

   - Inform the player about the range of the secret number.

5. Game loop

   ```rust
   loop {
   ```

   - A loop is initiated, which continues until the correct number is guessed.
   - The player is prompted to enter their guess.

   1. Reading and Parsing Input

      ```rust
      let mut guess = String::new();

      io::stdin()
              .read_line(&mut guess)
              .expect("Failed to read line.");
      ```

      - `String::new()`: Creates a new, empty string to store the user's input.
      - `io::stdin().read_line(&mut guess)`: Read the input from the user and stores it in the `guess` variable.

   2. Handling Invalid Input
      ```rust
      let guess: u32 = match guess.trim().parse() {
          Ok(num) => num,
          Err(_) => {
              println!("That doesn't seem like a number. Please enter a valid number:");
              continue;
          }
      };
      ```

   - `.trim().parse()`: Trims whitespace and attempts to parse the input into an unsigned 32-bit integer (`u32`).
   - If parsing fails, an error message is displayed, and the loop continues to prompt the player.

   3. Comparing the Guess

   ```rust
   match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too low! Try again:"),
        Ordering::Greater => println!("Too high! Try again:"),
        Ordering::Equal => {
            println!("Congratulations! You guessed the right number!");
            break;
        }
    }
   ```

   - `guess.cmp(&secret_number)`: Compares the guess with the secret number.
   - Based on the result, it either tells the player that their guess was too low, too high, or correct.
   - If guess is correct, a congratulatory message is displayed, and the game ends.

## Building and Running the Game

1. Building the Project
   - Run the following command to build the project:
     ```bash
     cargo build
     ```
2. Running the Game
   - After building, run the game with:
   ```bash
   cargo run
   ```
