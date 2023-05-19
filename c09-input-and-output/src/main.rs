use std::env;
use std::fs;
use std::io::Write;

fn main() {
    // Command-line arguments
    if env::args().len() <= 2 {
        println!("Program requires at least 2 arguments!");
        return;
    }

    for (index, argument) in env::args().enumerate() {
        println!("argument {index} is {argument}");
    }

    let arg2 = env::args().nth(2).unwrap();
    println!("arg2: {arg2}");

    // Reading from files
    let contents = fs::read_to_string("planets.txt").unwrap();
    println!("contents: {contents}");

    for line in contents.lines() {
        println!("line is {line}");
    }

    let contents = fs::read("planets.txt").unwrap();
    println!("contents: {contents:?}");

    // Writing to files
    let mut speech = String::new();
    speech.push_str("We choose to go to the Moon in this decate\n");
    speech.push_str("and do other things,\n");
    speech.push_str("not because they are easy,\n");
    speech.push_str("but because they are hard!");
    fs::write("speech.txt", speech);

    let mut file = fs::OpenOptions::new()
        .append(true)
        .open("planets.txt")
        .unwrap();
    file.write(b"\nPluto");

    // Challenge: Check to roster
    if env::args().len() < 2 {
        eprintln!("Program requires two arguments: <file path> <search name>");
        std::process::exit(1);
    }
    let file_path = env::args().nth(1).unwrap();
    let search_name = env::args().nth(2).unwrap();

    for line in fs::read_to_string(file_path).unwrap().lines() {
        if line == search_name {
            println!("{} did walk on the Moon!", search_name);
            return;
        }
    }

    println!("{} did NOT walk on the Moon... YET!", search_name);
}
