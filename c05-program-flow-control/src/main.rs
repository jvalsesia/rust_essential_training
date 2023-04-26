fn main() {
    // Conditional execution
    let x = 3;
    let y = 5;
    if x == 3 {
        println!("x is {x} and y is {y}");
    }

    // Multiple conditions
    if x > y {
        println!("{x} is GREATER than {y}");
    } else {
        if x < y {
            println!("{x} is LESS than {y}");
        } else {
            println!("{x} is EQUAL to {y}");
        }
    }

    if x > y {
        println!("{x} is GREATER than {y}");
    } else if x < y {
        println!("{x} is LESS than {y}");
    } else {
        println!("{x} is EQUAL to {y}");
    }

    // Conditional assignment
    let make_x_odd = true;
    let z = if make_x_odd { 1 } else { 2 };

    // if make_x_odd {
    //     z = 1;
    // } else {
    //     z = 2;
    // }
    println!("z is {z}");

    // Loops
    let mut count = 0;
    let result = loop {
        if count == 10 {
            break count * 10;
        }
        count += 1;
        println!("count is {count}");
    };
    println!("result is {result}");

    // While loops
    let mut count2 = 0;
    let letters = ['a', 'b', 'c'];
    while count2 < letters.len() {
        println!("letter is {}", letters[count2]);
        count2 += 1;
    }

    // For loops
    let message = ['h', 'e', 'l', 'l', 'o'];
    for item in message {
        println!("item is {item}");
    }

    for (index, item) in message.iter().enumerate() {
        println!("{index} - item is {item}");
    }

    // &
    for (index, &item) in message.iter().enumerate() {
        println!("{index} - item is {item}");
        if item == 'e' {
            break;
        }
    }

    for number in 0..10 {
        println!("{number}");
    }

    // Nested loops
    let mut matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];

    for row in matrix.iter() {
        for num in row.iter() {
            print!("{num}\t"); // tab \t
        }
        println!();
    }

    // iter_mut() to modify values iterating with array
    for row in matrix.iter_mut() {
        for num in row.iter_mut() {
            *num += 10; // dereference *
            print!("{num}\t");
        }
        println!();
    }

    // Challenge: Max, min, mean
    // mean = Sum of array elements / number of array elements
    let challenge_numbers = [1, 9, -2, 0, 23, 20, -7, 13, 37, 20, 56, -18, 20, 3];
    let mut max: i32;
    let mut min: i32;
    let mut mean: f64;

    min = challenge_numbers[0];
    max = challenge_numbers[0];
    mean = 0.0;
    for &n in challenge_numbers.iter() {
        if n < min {
            min = n;
        } else if n > max {
            max = n;
        }
        mean += n as f64;
    }
    mean /= challenge_numbers.len() as f64;

    println!("min: {min}");
    println!("max: {max}");
    println!("mean: {mean}");
    assert_eq!(max, 56);
    assert_eq!(min, -18);
    assert_eq!(mean, 12.5);
    println!("Tests passed!");
}
