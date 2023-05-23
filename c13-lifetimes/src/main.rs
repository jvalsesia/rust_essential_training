// Lifetime annotation syntax
// What it does it tell to compiler how the lifetimes of the input parameters
// relate to each other. This is telling Rust that the lifetime  of the returned
// reference will be as long as the lifetime of the two input parameters x and y.
// If x an y have different lifetimes the compiler will use the most restrictive
// or smaller of the two.
fn best_fuel<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Multiple lifetime annotations
fn really_best_fuel<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        x
    }
}

// Lifetime elision rules
// elision: the omission of a sound or syllable when speaking (as in I'm, let's, e ' en ).
fn get_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..index]; // found a space
        }
    }
    &s // no spaces found; input is a single word
}

// Struct lifetime annotation
struct Shuttle<'a> {
    name: &'a str,
}
impl<'a> Shuttle<'a> {
    fn send_transmission(&self, msg: &str) -> &str {
        println!("Transmitting message: {msg}");
        self.name
    }
}

fn main() {
    // The borrow checker
    // The borrow checker is a part of the Rust compiler that makes sure
    // he scope of variables to make sure all borrows are valid
    // This check helps to ensure that your program doesn't try to use a
    // reference after the thing it points to has gone out of scope.
    //
    // ' is the lifetime
    // 'a
    let propellant;
    // 'a
    let rp1 = String::from("RP-1");
    {
        // 'b
        propellant = &rp1;
    }
    // 'a
    println!("propellant is {}", propellant);

    // Lifetime annotation syntax
    let result;
    let propellant1 = String::from("RP-1");
    let propellant2 = String::from("LNG");
    result = best_fuel(&propellant1, &propellant2);
    println!("result: {result}");

    // Multiple lifetime annotations
    let result2;
    result2 = really_best_fuel(&propellant1, &propellant2);
    println!("result2: {result2}");

    // Lifetime elision rules
    let message = String::from("Greetings from Earth!");
    let first_word = get_first_word(&message);
    println!("first_word is {first_word}");

    // Struct lifetime annotation
    let vehicle = Shuttle { name: "Endeavour" };
    let sender = vehicle.send_transmission("Greeting from orbit!");
    println!("sender is {sender}");
}
