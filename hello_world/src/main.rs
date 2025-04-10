use core::num;
use std::io;

enum Operations {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Mean,
    Remainder
}

struct CalculationData {
    f_num: f32,
    s_num: f32,
    operation_type: Operations
}

fn main() {
    println!("Welcome to Project 1: The Calculator");

    println!("Please choose one of the following operations to perform:");
    println!("1-Addition");
    println!("2-Subtraction");
    println!("3-Multiplication");
    println!("4-Division");
    println!("5-Average/Mean");
    println!("6-Remainder");

    // variable declaration and initialization
    // mutability
    // result
    // error handling
    // placeholder
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Unable to read the input");
    println!("You have entered {input}");
    let choice = input.trim().parse::<i8>().unwrap();

    let mut first_number: f32 = 0.0;
    let mut second_number: f32 = 0.0;

    if choice == 5 {
        println!("Please enter all the numbers separated by commas and without spaces");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Unable to read the input");
        println!("You have entered {input}");

        let vec_num = input
            .trim()
            .split(',')
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        findMean(vec_num);
    }
    else {
        println!("Please enter the first number");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Unable to read the input");
        println!("You have entered {input}");
        first_number = input.trim().parse::<i32>().unwrap() as f32;

        println!("Please enter the second number");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Unable to read the input");
        println!("You have entered {input}");
        second_number = input.trim().parse::<i32>().unwrap() as f32;   
    }

    let mut result: f32 = 0.0;

    // flow control
    if choice == 1 {
        result = calculation(CalculationData { 
            f_num: first_number,
            s_num: second_number,
            operation_type: Operations::Addition
        });
    }
    else if choice == 2 {
        result = calculation(CalculationData { 
            f_num: first_number,
            s_num: second_number,
            operation_type: Operations::Subtraction
        });
    }
    else if choice == 3 {
        result = calculation(CalculationData { 
            f_num: first_number,
            s_num: second_number,
            operation_type: Operations::Multiplication
        });
    }
    else if choice == 4 {
        result = calculation(CalculationData { 
            f_num: first_number,
            s_num: second_number,
            operation_type: Operations::Division
        });
    }
    else if choice == 6 {
        let tuple_demo = (first_number as i64, second_number as f32);
        result = findRemainder(tuple_demo) as f32;
    }

    if choice != 5 {
        println!("Your result is {result}");
    }
}

fn calculation(data: CalculationData) -> f32 {
    match data.operation_type {
        Operations::Addition => {
            data.f_num + data.s_num
        },
        Operations::Subtraction => {
            data.f_num - data.s_num
        },
        Operations::Multiplication => {
            data.f_num * data.s_num
        },
        Operations::Division => {
            data.f_num / data.s_num
        },
        Operations::Mean => {
            (data.f_num + data.s_num) / 2.0
        },
        Operations::Remainder => {
            data.f_num / data.s_num
        }
    }
}

fn findMean(values: Vec<i32>) {
    let mut sum = 0;

    for num in values.iter() {
        sum = sum + num;
    }

    let result = sum / values.len() as i32;
    println!("The mean of the given values = {result}")
}

fn findRemainder(values: (i64, f32)) -> i32 {
    let result = values.0 % values.1 as i64;
    result as i32
}
