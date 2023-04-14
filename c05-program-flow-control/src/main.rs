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
    let letters =['a','b','c'];
        while count2 < letters.len() {
        println!("letter is {}", letters[count2]);
        count2 += 1;
    }


}
