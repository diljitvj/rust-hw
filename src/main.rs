// To read the user input, io library is needed. We need to bring the library to the scope
use std::io;

fn main() {
    println!("Guess the game.");
    println!("Please input your guess : ");

    let mut guess = String::new();
    // mut creates a mutable variable
    // String::new() creates a new instance of String

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    // Read the user input and save the value in the variable guess
    // .expect handles the error if any

    println!("You guessed {}", guess);
    // {} is the placeholder for a variable
}
