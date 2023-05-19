fn main() {
    // Variable scope

    // Scope: Region of the program where a variable is valid
    // variable bindings are constrained to live within a block of code {}
    let moon = "moon";
    if true {
        let planet = "Earth";
        println!("planet is {planet}");
        println!("moon is {moon}");
    }
    // after this block of code
    // planet not found in this scope
    // println!("planet is {planet}");
    // but moon is
    println!("planet is {moon}");

    // Shadowing variables
    // Declare a new variable with the same name as an existing variable
    // New variables "shadows" the previous one
    let new_planet = "Jupiter";
    {
        println!("new_planet is {new_planet}");
        let new_planet = "Saturn";
        println!("new_planet is {new_planet}");
        let mut new_planet = 4; // mutability
        println!("new_planet is {new_planet}");
        new_planet = 5;
        println!("new_planet is {new_planet}");
    }
    println!("new_planet is {new_planet}");

    // Stack and heap memory
    // Stack: boxes of different sizes
    // - Values stored in sequential order
    // - Data added and removed as Last In, First Out (LIFO)
    // - Data can be pushed and popped very quickly
    // - All data must have a known, fixed size

    // Heap: shelves in wareshouse that store boxes of different sizes
    // - Adding and acessing data is slower that the Stack
    // - Dynamically add and remove data

    // Pointer: is a data type that stores memory address
    // Pointer is the inventory sheet that tells you where the specific box is stored in the shelf in the warehouse

    // String data type
    // String Literal:
    // - Hard-coded into the executable
    // - Imutable
    // - Must be known before compilation

    // String Type:
    // - Allocated on the heap
    // - Mutable
    // - Dynamically generated at runtime

    // Copying data on the stack occurs implicitly, whereas cloning data on the heap must be done explicitly using the clone method.

    let mut message = String::from("Earth");
    println!("message: {message}");
    message.push_str(" is home");
    println!("message: {message}");

    // Ownership
    // Explicit Allocation and Deallocation:
    // - Programmer is responsible for memory managment
    // Garbage Collection
    // - waste memory
    // In Rust Ownership
    // - variables are responsible for freeing their own resources

    // Moving, cloning and copying data
    // Moving
    // "Mercury"    String is on the Heap
    // inner_planet and outer_planet are both in the Stack
    // 1st inner_planet is point to "Mercury" the Heap
    // 2nd outer_planet is copying the inner_planet
    // 3rd inner_planet is removed from Stack
    let outer_planet: String;
    {
        let inner_planet = String::from("Mercury");
        println!("inner_planet: {inner_planet}");
        outer_planet = inner_planet;
        // here inner_planet was moved and no longer exists
        // println!("inner_planet: {inner_planet}");
    }
    println!("outer_planet: {outer_planet}");

    // Cloning
    let outer_planet_2: String;
    {
        let mut inner_planet_2 = String::from("Mercury");
        println!("inner_planet_2: {inner_planet_2}");
        // here inner_planet_2 was cloned and clear
        outer_planet_2 = inner_planet_2.clone();
        inner_planet_2.clear();
        println!("inner_planet_2: {inner_planet_2}");
    }
    println!("outer_planet_2: {outer_planet_2}");

    // Moving and Cloning does not work for Integer
    // In Integers data types Rust just copy the

    // Copying
    let outer_planet_3: i32;
    {
        let mut inner_planet_3 = 4;
        println!("inner_planet_3: {inner_planet_3}");
        // here inner_planet_3 was copied because i32 does not have clone
        // so i32, f64 do not use Heap... just Stack
        outer_planet_3 = inner_planet_3;
        inner_planet_3 += 1;
        println!("inner_planet_3: {inner_planet_3}");
    }
    println!("outer_planet_3: {outer_planet_3}");

    // Transferring Ownership

    let rocket_fuel = 1;
    process_fuel(rocket_fuel);
    println!("rocket fuel: {rocket_fuel}");

    let rocket_fuel_2 = String::from("RP-1");
    // here we need ot use clone beacuase rocket_fuel_2 is String and use HEAP
    process_fuel_string(rocket_fuel_2.clone()); 
    println!("rocket fuel 2: {rocket_fuel_2}");

    let rocket_fuel_3 = String::from("RP-2");
    // here we need to return a string tranferred in return
    let rocket_fuel_3 = process_fuel_string_tranferred(rocket_fuel_3); 
    println!("rocket fuel 3: {rocket_fuel_3}");
    
}

fn process_fuel(mut propellant: i32) {
    propellant += 1;
    println!("processing propellant: {propellant}");
}

fn process_fuel_string(propellant: String) {
    println!("processing propellant: {propellant}");
}

fn process_fuel_string_tranferred(propellant: String) -> String{
    println!("processing propellant: {propellant}");
    let new_fuel = String::from("LNG");
    new_fuel
}