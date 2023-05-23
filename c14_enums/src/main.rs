#[derive(Debug)]
enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64, f64),
}

// Enum menthods
impl Shape {
    fn get_perimeter(&self) -> f64 {
        match *self {
            Shape::Circle(r) => r * 2.0 * std::f64::consts::PI,
            Shape::Rectangle(w, h) => (2.0 * w) + (2.0 * h),
            Shape::Triangle(a, b, c) => a + b + c,
        }
    }
}

// Challenge: Represent a location
enum Location {
    Unknown,
    Anonymous,
    Know(f64, f64), // latitude, longitude
}
impl Location {
    fn display(&self) {
        match *self {
            Location::Unknown => println!("Unknown location"),
            Location::Anonymous => println!("Anonymous location"),
            Location::Know(lat, lon) => println!("{lat}, {lon }"),
        }
    }
}

fn main() {
    // Define enums
    let my_shape = Shape::Rectangle(1.2, 3.4);
    println!("my_shape is {my_shape:?}");

    // Match operator
    match my_shape {
        Shape::Circle(r) => println!("Circle with radius {r}"),
        Shape::Rectangle(w, h) => println!("Rectangle {w} x {h}"),
        Shape::Triangle(a, b, c) => println!("Triangle {a}, {b}, {c}"),
    }

    // Match with default placeholder
    // Always include _ to the end
    let my_number = 4u8;
    let result = match my_number {
        0 => "zero",
        1 => "one",
        2 => "two",
        3 => "three",
        _ => {
            println!("{my_number} did not match");
            "something else"
        }
    };
    println!("result is {result}");

    // Enum menthods
    let perimeter = my_shape.get_perimeter();
    println!("perimeter is {perimeter}");

    // Option<T> enum
    // None and Some
    let countdown = [5, 4, 3, 2, 1];
    let number = countdown.get(5);
    println!("number is {number:?}"); // will return None because 5 is out of the bound
    let number2 = countdown.get(4);
    println!("number2 is {number2:?}"); // will return Some because 4 is in the bound
    let number3 = countdown.get(1);
    let number3 = number3.unwrap() + 1;
    println!("number3 is {number3:?}"); // will return (countdown[1] == 4) + 1

    let number4 = countdown.get(5); // index 5 is out of the bound
    let number4 = number4.unwrap_or(&0) + 1; // if out of the bound we can't unwrap without a default value
    println!("number4 is {number4:?}"); // will return (unwrap_or == 0) + 1

    // Matchin Option<T>
    let number5 = countdown.get(2);
    let number5 = match number5 {
        Some(number5) => number5 + 1,
        None => 0,
    };
    println!("number5 is {number5:?}");

    // If-let syntax
    let number6 = Some(14);
    if let Some(14) = number6 {
        println!("fourteen");
    }

    // Challenge: Represent a location
    let address = Location::Unknown;
    address.display();
    let address = Location::Anonymous;
    address.display();
    let address = Location::Know(28.608295, -80.604177);
    address.display();
}
