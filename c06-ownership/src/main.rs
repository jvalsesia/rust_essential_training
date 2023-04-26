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
    


}
