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
}
