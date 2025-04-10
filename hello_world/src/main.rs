use std::io;

fn main() {
    println!("Welcome to Project 1: The Calculator");

    println!("Please choose one of the following operations to perform:");
    println!("1-Addition");
    println!("2-Subtraction");
    println!("3-Multiplication");
    println!("4-Division");

    // variable declaration and initialization
    // mutability
    // result
    // error handling
    // placeholder
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Unable to read the input");
    println!("You have entered {input}");
    let mut choice = input.trim().parse::<i8>().unwrap();

    println!("Please enter the first number");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Unable to read the input");
    println!("You have entered {input}");
    let mut first_number = input.trim().parse::<i32>().unwrap() as f32;

    println!("Please enter the second number");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Unable to read the input");
    println!("You have entered {input}");
    let mut second_number = input.trim().parse::<i32>().unwrap() as f32;

    println!("You have entered {first_number} and {second_number}");
    let mut result: f32 = 0.0;

    // flow control
    if choice == 1 {
        result = first_number + second_number;
    }
    else if choice == 2 {
        result = first_number - second_number;
    }
    else if choice == 3 {
        result = first_number * second_number;
    }
    else if choice == 4 {
        result = first_number / second_number;
    }
    else {
        println!("Please enter a number from 1 to 4");
    }

    println!("Your result is {result}");
}
