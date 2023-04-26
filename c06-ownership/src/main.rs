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
}
