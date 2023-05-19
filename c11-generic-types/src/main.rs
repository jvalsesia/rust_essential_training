use std::mem;

#[derive(Debug)]
struct Rectangle<T> {
    width: T,
    height: T,
}

#[derive(Debug)]
struct RectangleA<T, U> {
    width: T,
    height: U,
}

#[derive(Debug, Clone)]
struct Shuttle {
    name: String,
    crew_size: u8,
    propellant: f64,
}

impl<T, U> RectangleA<T, U> {
    fn get_width(&self) -> &T {
        &self.width
    }
}
// Generict methods definitions
impl RectangleA<u8, u8> {
    fn get_perimeter(&self) -> u8 {
        2 * self.height + 2 * self.width
    }
}

// Generict functions definitions
fn get_biggest<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

fn main() {
    // Generic struct definitions
    // <T>, <T,U>
    // T and U, those two arguments could be the same or different data types.
    // Using generics, you can define a single struct that is flexible to store its values as any data type. 
    // This allows you to avoid having to write multiple struct definitions for different data types.

    let rect = Rectangle {
        width: 3.8,
        height: 5.8,
    };
    println!("rect is {rect:?}");

    let rect2 = Rectangle {
        width: 3,
        height: 5,
    };
    println!("rect2 is {rect2:?}");

    let rect3 = RectangleA {
        width: 3,
        height: 5.8,
    };

    println!("rect3 is {rect3:?}");
    println!("rect3 width is {}", rect3.get_width());

    // Generict methods definitions
    let rect4 = RectangleA {
        width: 10u8,
        height: 12u8,
    };
    println!("perimeter is {}", rect4.get_perimeter());

    // Generic functions definitions
    println!("biggest is {}", get_biggest(30, 3));
    println!("biggest is {}", get_biggest(30.9, 78.1));

    // Box data type
    // The Box data type consists of a pointer on the stack that points to a chunk of memory 
    // on the heap that has been allocated for the data. When the box goes out of scope, 
    // it will automatically drop and deallocate the memory it was using on the heap.
    let mut vehicle = Shuttle {
        name: String::from("Endeavour"),
        crew_size: 7,
        propellant: 0.0,
    };
    println!(
        "Vehicle size on stack: {} bytes",
        mem::size_of_val(&vehicle)
    );

    let boxed_vehicle: Box<Shuttle> = Box::new(vehicle);
    println!(
        "boxed_vehicle size on stack: {} bytes",
        mem::size_of_val(&boxed_vehicle)
    );
    // Dereference operator *
    println!(
        "boxed_vehicle size on heap: {} bytes",
        mem::size_of_val(&*boxed_vehicle)
    );

    let unboxed_vehicle: Shuttle = *boxed_vehicle;
    println!(
        "unboxed_vehicle size on stack: {} bytes",
        mem::size_of_val(&unboxed_vehicle)
    );

    // Challenge: Sum boxes
    let one = Box::new(1);
    let two = Box::new(2);
    assert_eq!(*sum_boxes(one, two), 3);

    let pi = Box::new(3.14159);
    let e = Box::new(2.71828);
    assert_eq!(*sum_boxes(pi, e), 5.85987);
    println!("Tests passed!");
}

// Challenge: Sum boxes
fn sum_boxes<T: std::ops::Add<Output = T>>(a: Box<T>, b: Box<T>) -> Box<T> {
    Box::new(*a + *b)
}
