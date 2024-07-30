use std::io;

fn average(numbers: &[i32]) -> f64 {
    let sum: i32 = numbers.iter().sum();
    let count = numbers.len() as f64;
    sum as f64 / count
}

fn main() {
    let mut input = String::new();
    
    // Prompt the user to enter the number of elements
    println!("Enter the number of elements:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let num_elements: usize = input.trim().parse().expect("Please enter a valid number");

    // Initialize a vector to store the numbers
    let mut numbers = Vec::new();

    // Prompt the user to enter each number
    for _ in 0..num_elements {
        input.clear();
        println!("Enter a number:");
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let number: i32 = input.trim().parse().expect("Please enter a valid number");
        numbers.push(number);
    }

    // Calculate the average and display the result
    let result = average(&numbers);
    println!("The average is: {}", result);
}
