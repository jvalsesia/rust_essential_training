fn main() {
    // Borrowing references
    // tranferring ownership is HEAP
    // borrowing is STACK (volatile)
    let rocket_fuel = String::from("RP-1");

    let (rocket_fuel, length) = process_fuel(rocket_fuel);
    println!("rocket fuel: {rocket_fuel} length: {length}");

    let rocket_fuel_2 = String::from("RP-2");
    let length2 = process_fuel_2(&rocket_fuel_2);
    println!("rocket fuel 2: {rocket_fuel_2} length: {length2}");

    // Mutable references
    let mut rocket_fuel_3 = String::from("RP-3");
    let length3 = process_fuel_3(&mut rocket_fuel_3);
    println!("rocket fuel 3: {rocket_fuel_3} length: {length3}");

    // Slices
    let message = String::from("Greetings from Earth!");
    println!("message: {message}");
    let last_word = &message[15..];
    println!("last_word: {last_word}");

    let planets = [1, 2, 3, 4, 5, 6, 7, 8];
    let inner_planets: &[i32] = &planets[..4];
    println!("inner_planets are: {inner_planets:?}");

    // Slices are function parameters
    let message2 = String::from("Greetings from Mars!");
    let first_word = get_first_word(&message2);
    println!("first_word is: {first_word}");

    // Write a function tom remove leading and trailing spaces from a string
    let test1 = "We need more space.";
    assert_eq!(trim_spaces(test1), "We need more space.");

    let test2 = String::from("   There's space in front.");
    assert_eq!(trim_spaces(&test2), "There's space in front.");

    let test3 = String::from("There's space to the rear. ");
    assert_eq!(trim_spaces(&test3[..]), "There's space to the rear.");

    let test4 = "  We're surrounded by space!    ";
    assert_eq!(trim_spaces(test4), "We're surrounded by space!");

    let test5 = "     ";
    assert_eq!(trim_spaces(test5), "");

    let test6 = "";
    assert_eq!(trim_spaces(test6), "");

    let test7 = " 🚀 ";
    assert_eq!(trim_spaces(test7), "🚀");
    println!("Tests passed!");
}

// &str is string slice
// A string reference will point to an actual string on the stack, which in turn points to and owns the string data that lives on the heap. 
// A slice will only store a pointer to the heap data.
// A slice only stores a pointer to the heap data, along with length information. 
// The slice doesn't need to keep track of capacity because the slice will never own anything on the heap.
fn get_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..index]; // found a space
        }
    }
    &s // no spaces found
}

// Here we are tranferring the ownership of propellant to process_fuel
fn process_fuel(propellant: String) -> (String, usize) {
    println!("processing propellant: {propellant}");
    let length = propellant.len();
    (propellant, length)
}

// Here we are borrowing (&) propellant to process_fuel_2
fn process_fuel_2(propellant: &String) -> usize {
    println!("processing propellant: {propellant}");
    let length = propellant.len();
    length
}

// Here we are mutatating borrowing (&) propellant to process_fuel_3
// When using a mutable reference, you cannot create other references
// Prevents data races
fn process_fuel_3(propellant: &mut String) -> usize {
    propellant.push_str(" is highly flammable!");
    println!("processing propellant: {propellant}");
    let length = propellant.len();
    length
}

// Write a function tom remove leading and trailing spaces from a string
fn trim_spaces(s: &str) -> &str {
    let mut start = 0;
    for (index, character) in s.chars().enumerate() {
        if character != ' ' {
            start = index;
            break;
        }
    }

    let mut end = 0;
    for (index, character) in s.chars().rev().enumerate() {
        if character != ' ' {
            end = s.len() - index;
            break;
        }
    }
    &s[start..end]
}
