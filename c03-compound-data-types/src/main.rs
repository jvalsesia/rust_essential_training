
fn main() {
    // Arrays
    let mut letters = ['a', 'b', 'c'];
    letters[0] = 'x';
    let first_letter = letters[0];
    println!("first letter is {first_letter}");

    let numbers: [i32; 5];
    numbers = [0; 5];
    let last_number = numbers[4];
    let len = numbers.len();
    println!("last number is {last_number} - array len {len}");


    // Multidimensional arrays
    let parking_lot = [[1,2,3],[4,5,6]];
    let number = parking_lot[0][2];
    println!("number is {number}");

    // A 2-dimensional array whose elements are all the value 1.
    let x = [[1; 2]; 3];
    println!("x is {x:?}");


    // 10 x 20 x 5 full of zeros
    let garage = [[[0; 10]; 20]; 5];
    println!("garage is {garage:?}");

    let lot = garage[1][1][1];
    println!("lot is {lot}");

    // Tuples
    // tuples are ordered
    let mut stuff =(10, 3.14, 'x');
    stuff.0 += 3;
    let first_item = stuff.0;
    println!("first_item is {first_item}");

    let (a,b,c) = stuff;
    println!("a is {a}");
    println!("b is {b}");
    println!("c is {c}");

    

}
