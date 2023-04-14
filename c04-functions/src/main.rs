fn main() {
    say_hello();

    let result = square(10);
    println!("result is {result}");
    let result2 = square2(10);
    println!("result2 is {result2:?}");

    let firenheit = celsius_to_fahrenthiet(23.0);
    println!("firenheit is {firenheit:?}");
}

// Function
fn say_hello() {
    println!("hello!");
    say_a_number(76);
    say_the_sum(1, 3);
    let x = 10;
    let y = 30;
    say_the_sum(x, y);
    say_a_number(x as i32);
}

// Function Parameters
fn say_a_number(number: i32) {
    println!("number is {number}");
}

fn say_the_sum(a: u8, b: u8) {
    let sum = a + b;
    println!("sum is {sum}");
}

// Statements vs. expressions
//         expression
//           _____
// let sum = a + b;
//_________________
//  statement

// Function return values
fn square(x: i32) -> i32 {
    println!("squaring {x}");
    x * x
}

// Function to return 2 values in tuple
fn square2(x: i32) -> (i32, i32) {
    println!("squaring {x}");
    return (x, x * x);
}

// Unit Data Type -> ()

// C to F
fn celsius_to_fahrenthiet(celsius: f64) -> f64 {
    println!("celsius {celsius}");

    1.8 * celsius + 32.0
}

#[test]
fn test_celsius_to_fahrenthiet() {
    // Test case 1: 0 Celsius is equal to 32 Fahrenheit
    let celsius_1 = 0.0;
    let expected_fahrenthiet_1 = 32.0;
    assert_eq!(celsius_to_fahrenthiet(celsius_1), expected_fahrenthiet_1);

    // Test case 2: 100 Celsius is equal to 212 Fahrenheit
    let celsius_2 = 100.0;
    let expected_fahrenthiet_2 = 212.0;
    assert_eq!(celsius_to_fahrenthiet(celsius_2), expected_fahrenthiet_2);

    // Test case 3: -40 Celsius is equal to -40 Fahrenheit
    let celsius_3 = -40.0;
    let expected_fahrenthiet_3 = -40.0;
    assert_eq!(celsius_to_fahrenthiet(celsius_3), expected_fahrenthiet_3);
}
