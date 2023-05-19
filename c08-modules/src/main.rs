use rand::{random, thread_rng, Rng};
use std::io; // from crates rand

fn main() {
    // Standart input
    let mut buffer = String::new();
    println!("Enter a messasge:");

    // io::stdin().read_line(&mut buffer);

    match io::stdin().read_line(&mut buffer) {
        Ok(n) => {
            println!("{n} bytes read");
            println!("buffer is: {buffer}");
        }
        Err(error) => println!("error: {error}"),
    }

    // Parse strings
    //  let number = buffer.trim().parse::<i32>().unwrap();
    let number: i32 = buffer.trim().parse().unwrap();
    println!("number + 1 is {}", number + 1);

    // Crates
    // https://crates.io
    // cargo add rand
    let num = random::<f64>();
    // ::<f64> This "turbofish" operator tells the parse function to convert the contents of the String buffer to an f64 value.
    println!("num is {num}");

    let num = thread_rng().gen_range(1..100);
    println!("num is {num}");

    // Challenge: Higher or lower
    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("I'm thinking of a number between 1 and 100...");
    println!("Guess the number:");
    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input line.");
        let guess: u32 = guess.trim().parse().expect("Failed to parse the guess.");

        if guess > secret_number {
            println!("\n{} is too high! Guess lower:", guess);
        } else if guess < secret_number {
            println!("\n{} is too low! Guess higher:", guess);
        } else {
            println!("\nYou got it! The secret number was {}.", secret_number);
            break;
        }
    }
}
