// To read the user input, io library is needed. We need to bring the library to the scope
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the game.");
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess : ");

        let mut guess = String::new();
        // mut creates a mutable variable
        // String::new() creates a new instance of String

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        // Read the user input and save the value in the variable guess
        // .expect handles the error if any

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };
        // trim method eliminates the \n character
        // guess variable has to be converted to a integer to compare it with the secret number
        // The previous value of guess has been shadowed by the new value after type casting to integer
        // The same variable name can be used, This is called Shadowing
        // Error can be handled using the match expression
        // Continue tells the program to go to the next iteration

        println!("You guessed {}", guess);
        // {} is the placeholder for a variable

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
