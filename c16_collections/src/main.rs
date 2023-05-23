use std::{collections::HashMap, env, fs};

fn main() {
    // Vectors
    // - Collections of elements with same data type
    // - Elements are stored in order
    // - Items can be dynamically added or removed
    // - Stored in heap memory

    let mut astronauts: Vec<String> = Vec::new();
    astronauts.push(String::from("Alan Shepard"));
    astronauts.push(String::from("Gus Grissom"));
    astronauts.push(String::from("John Glenn"));

    println!("astronauts is {astronauts:?}");

    let last = astronauts.pop();
    println!("last is {last:?}");

    // let third = &astronauts[2];
    let third = astronauts.get(2); // 2 was removed in pop before
    println!("third is {third:?}");

    let countdown = vec![5, 4, 3, 2, 1, 0];
    println!("countdown is {countdown:?}");

    // Hashmaps
    // - Store dagta in key->value pairs
    // - Use keys to lookup corresponding values
    // - key->value mapping is one way
    // - Uses a hash function to determine how to store data
    // - Keys and values can be different data types
    // - All keys must have the same data type
    // - All values must have the same data type
    // - Each key can only have one value associated with it at a time
    let mut mission_flown = HashMap::new();
    mission_flown.insert("Chris Hadfield", 3);
    mission_flown.insert("Doug Hurley", 3);
    mission_flown.insert("Kayla Barron", 0);
    mission_flown.insert("Kayla Barron", 1); // updating
    mission_flown.entry("Kayla Barron").or_insert(2); // updating if not exist
    mission_flown.entry("Julio Valsesia").or_insert(2); // updating if not exist
    println!("mission_flown is {mission_flown:?}");

    let kayla = mission_flown.entry("Kayla Barron").or_insert(0);
    *kayla += 1;

    let barrow_missions = mission_flown.get("Kayla Barron");
    println!("barrow_missions is {barrow_missions:?}");

    // Challenge: Count words
    // read file and build vector of individual words
    let contents = match env::args().nth(1) {
        Some(f) => match fs::read_to_string(f) {
            Ok(s) => s.to_lowercase(),
            Err(e) => {
                eprintln!("Could not read file: {}", e);
                std::process::exit(1);
            }
        },
        None => {
            eprintln!("Program requires an argument: <file path>");
            std::process::exit(2);
        }
    };
    let all_words = contents.split_whitespace().collect::<Vec<&str>>();

    // count how many times each unique word occurs
    let mut word_counts: HashMap<&str, u32> = HashMap::new();
    for word in all_words.iter() {
        *word_counts.entry(word).or_insert(1) += 1;
    }

    // determine the most commonly used word(s)
    let mut top_count = 0u32;
    let mut top_words: Vec<&str> = Vec::new();
    for (&key, &val) in word_counts.iter() {
        if val > top_count {
            top_count = val;
            top_words.clear();
            top_words.push(key);
        } else if val == top_count {
            top_words.push(key);
        }
    }

    // display results
    println!("Top word(s) occurred {} times:", top_count);
    for word in top_words.iter() {
        println!("{}", word);
    }
}
