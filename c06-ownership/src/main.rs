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
    let newPlanet = "Jupiter";
    {
        println!("newPlanet is {newPlanet}");
        let newPlanet = "Saturn";
        println!("newPlanet is {newPlanet}");
        let mut newPlanet = 4; // mutability
        println!("newPlanet is {newPlanet}");
        newPlanet = 5;
        println!("newPlanet is {newPlanet}");
    }
    println!("newPlanet is {newPlanet}");



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
    // Pointer is the inventory sheet that tells you where the specific box is stored in the shelf in the warehouseß



}
