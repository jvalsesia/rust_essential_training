use std::fs;
use std::io;

use rand::Rng;

// Propagating errors
fn read_and_combine(f1: &str, f2: &str) -> Result<String, io::Error> {
    let mut s1 = match fs::read_to_string(f1) {
        Ok(s) => s,
        Err(e) => return Err(e),
    };
    let s2 = match fs::read_to_string(f2) {
        Ok(s) => s,
        Err(e) => return Err(e),
    };

    s1.push('\n');
    s1.push_str(&s2);
    Ok(s1)
}

fn read_and_combine_2(f1: &str, f2: &str) -> Result<String, io::Error> {
    let mut s1 = fs::read_to_string(f1)?;
    let s2 = match fs::read_to_string(f2) {
        Ok(s) => s,
        Err(e) => return Err(e),
    };

    s1.push('\n');
    s1.push_str(&s2);
    Ok(s1)
}

fn read_and_combine_3(f1: &str, f2: &str) -> Result<String, io::Error> {
    let mut s1 = fs::read_to_string(f1)?;
    let s2 = fs::read_to_string(f2)?;

    s1.push('\n');
    s1.push_str(&s2);
    Ok(s1)
}

fn main() {
    // Unrecoverable errors
    // Types of errors:
    // - Recoverable: example, file not found error, handle with Result<T,E>
    // - Unrecoverable: example, index beyond array bounds, handle with panic!
    //panic!("Houston, we have a problem!");

    let countdown = [5, 4, 3, 2, 1, 0];

    for count in countdown.iter() {
        println!("T-minus {count}");
        // let x = 1 / count;
        // println!("x: {x}"); // panic! when count == 0
    }

    // Result<T,E> enum
    let contents = fs::read_to_string("answer_to_the_ultimate_question.txt").unwrap();
    println!("contents is: {contents:?}");

    // let contents = fs::read_to_string("the_ultimate_question.txt")
    //     .expect("Nobody know the ultimate question!");

    // Matching Result<T,E> to recover from errors
    let result = fs::read_to_string("the_ultimate_question.txt");
    let contents2 = match result {
        Ok(message) => message,
        Err(error) => match error.kind() {
            io::ErrorKind::NotFound => String::from("File no found!"),
            io::ErrorKind::PermissionDenied => String::from("Permission denied"),
            _ => panic!("Another typer of error: {error:?}"),
        },
    };
    println!("contents2 is: {contents2:?}");

    // Propagating errors
    let result2 = read_and_combine("planets.txt", "dwarf_planets.txt");
    match result2 {
        Ok(s) => println!("result2 is ...\n{s}"),
        Err(e) => println!("There was an error for result2: {e}"),
    }

    let result3 = read_and_combine("planetssssss.txt", "dwarf_planets.txt");
    match result3 {
        Ok(s) => println!("result3 is ...\n{s}"),
        Err(e) => println!("There was an error for result3: {e}"),
    }

    let result4 = read_and_combine_2("planets.txt", "dwarf_planets.txt");
    match result4 {
        Ok(s) => println!("result4 is ...\n{s}"),
        Err(e) => println!("There was an error for result4: {e}"),
    }

    let result5 = read_and_combine_3("planets.txt", "dwarf_planets.txt");
    match result5 {
        Ok(s) => println!("result5 is ...\n{s}"),
        Err(e) => println!("There was an error for result5: {e}"),
    }

    // Challenge: Handle errors
    let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("secret_number: {secret_number}");
    println!("I'm thinking of a number between 1 and 100...");
    println!("Guess the number:");
    loop {
        let mut buffer = String::new();
        let guess = match io::stdin().read_line(&mut buffer) {
            Ok(_) => match buffer.trim().parse::<u32>() {
                Ok(value) => value, // success
                Err(_) => {
                    println!("\nFailed to parse input. Guess again:");
                    continue;
                }
            },
            Err(_) => {
                println!("\nFailed to read input. Guess again:");
                continue;
            }
        };

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
