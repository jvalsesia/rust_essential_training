// Trait bounds
use std::any;
use std::fmt;

// Derive traits
#[derive(PartialEq, PartialOrd)]
struct Satellite {
    name: String,
    velocity: f64,
}

struct SpaceStation {
    name: String,
    crew_size: u8,
    altitude: u32,
}

#[derive(Debug)]
struct Ufo {
    name: String,
    velocity: f64,
}

// Implementing traits
trait Description {
    fn describe(&self) -> String;
}

// Default trait implementation
trait Description2 {
    fn describe(&self) -> String {
        String::from("an object flying through space!")
    }
}

impl Description for Satellite {
    fn describe(&self) -> String {
        format!(
            "the {} flying at {} miles per second!",
            self.name, self.velocity
        )
    }
}
// Default trait implementation
impl Description2 for Ufo {}

impl Description for SpaceStation {
    fn describe(&self) -> String {
        format!(
            "the {} flying at {} miles high with {} crew member aboard!",
            self.name, self.altitude, self.crew_size
        )
    }
}

// Trait bounds
fn print_type<T: fmt::Debug>(item: T) {
    println!("{:?} is {} ", item, any::type_name::<T>());
}

// Multiple trait bounds
//fn compare_and_print<T: fmt::Display + PartialEq + From<U>, U: fmt::Display + PartialEq + Copy>(a: T, b: U,) {
fn compare_and_print<T, U>(a: T, b: U)
where
    T: fmt::Display + PartialEq + From<U>,
    U: fmt::Display + PartialEq + Copy,
{
    if a == T::from(b) {
        println!("{} is equal to {}", a, b)
    } else {
        println!("{} is NOT equal to {}", a, b)
    }
}

fn main() {
    let hubble = Satellite {
        name: String::from("Hublle Telescope"),
        velocity: 4.72,
    };

    let gps = Satellite {
        name: String::from("GPS Telescope"),
        velocity: 2.42,
    };
    let iss = SpaceStation {
        name: String::from("International Space Station"),
        crew_size: 6,
        altitude: 254,
    };
    // Implementing traits
    println!("hubble: {}", hubble.describe());
    println!("iss: {}", iss.describe());

    // Default trait implementation
    let ufo = Ufo {
        name: String::from("Maars Shuttle"),
        velocity: 999.99,
    };
    println!("ufo: {:?} - description: {}", ufo, ufo.describe());

    // Derive traits
    println!("hubble == gps is {}", hubble == gps); // PartialEq
    println!("hubble > gps is {}", hubble > gps); // PartialOrd

    // Trait bounds
    print_type(10);
    print_type(10.0);
    print_type("ten");
    print_type([10]);

    // Multiple trait bounds
    compare_and_print(1.0, 1);
    compare_and_print(1.1, 1);
}
