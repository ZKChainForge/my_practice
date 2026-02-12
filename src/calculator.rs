use std::io;

pub fn run() {
    let mut input = String::new();

    println!("Enter first number:");
    io::stdin().read_line(&mut input).expect("Failed to read");
    let num1: f64 = input.trim().parse().expect("Invalid number");

    input.clear();

    println!("Enter second number:");
    io::stdin().read_line(&mut input).expect("Failed to read");
    let num2: f64 = input.trim().parse().expect("Invalid number");

    println!("Sum: {}", num1 + num2);
    println!("Difference: {}", num1 - num2);
    println!("Product: {}", num1 * num2);
    println!("Quotient: {}", num1 / num2);
}
