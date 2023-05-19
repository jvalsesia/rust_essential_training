#[derive(Debug, Clone)]
struct Shuttle {
    name: String,
    crew_size: u8,
    propellant: f64,
}

struct Color(u8, u8, u8); // RGB

struct Point(u8, u8, u8); // XYZ

struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    fn get_area(&self) -> f64 {
        &self.width * &self.height
    }

    fn scale(&mut self, scalar: f64) {
        self.width *= scalar;
        self.height *= scalar;
    }

    fn new(width: f64, height: f64) -> Rectangle {
        Rectangle { width, height }
    }
}

impl Shuttle {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn add_fuel(&mut self, gallons: f64) {
        self.propellant += gallons;
    }

    // Associated functions
    fn new(name: &str) -> Shuttle {
        Shuttle {
            name: String::from(name),
            crew_size: 7,
            propellant: 0.0,
        }
    }
}

fn get_y(p: Point) -> u8 {
    p.1
}

fn main() {
    // Defining structs
    let mut vehicle = Shuttle {
        name: String::from("Endeavour"),
        crew_size: 7,
        propellant: 0.0,
    };

    println!("vehicle name: {}", vehicle.name);

    vehicle.name = String::from("Atlantis");

    println!("Shuttle: {:?}", vehicle);

    // Struct update syntax
    let vehicle2 = Shuttle {
        name: String::from("Discovery"),
        ..vehicle // .. will copy the remaing vehicle fields
    };

    println!("vehicle2: {:?}", vehicle2);

    let vehicle3 = Shuttle {
        ..vehicle.clone() // clone vehicle deriving Clone
    };

    println!("vehicle3: {:?}", vehicle3);

    // Struct methods
    // using impl Shuttle
    let vehicle_name = vehicle.get_name();

    println!(
        "vehicle_name: {} - propellant: {}",
        vehicle_name, vehicle.propellant
    );

    let mut vehicle2 = Shuttle {
        name: String::from("Columbia"),
        crew_size: 7,
        propellant: 0.0,
    };
    vehicle2.add_fuel(1000.0);
    println!(
        "vehicle2 name: {} - crew: {} - propellant: {}",
        vehicle2.get_name(),
        vehicle2.crew_size,
        vehicle2.propellant
    );

    // Associated functions
    // Associated function are related to a specific struct data type; however, 
    // they cannot access the data within a specific instance of that struct.
    let vehicle3 = Shuttle::new("Challenger");
    println!(
        "vehicle3 name: {} - crew: {} - propellant: {}",
        vehicle3.get_name(),
        vehicle3.crew_size,
        vehicle3.propellant
    );

    // Tuple structs
    let red = Color(255, 0, 0);
    println!("Firt red value is {}", red.0);

    let coord = Point(38, 48, 87);
    let y = get_y(coord);
    println!("Point y is {}", y);

    // Challenge: Represents shapes
    let mut rect = Rectangle::new(1.2, 3.4);
    assert_eq!(rect.get_area(), 4.08);
    rect.scale(0.5);
    assert_eq!(rect.get_area(), 1.02);
    println!("Tests passed!");
}
