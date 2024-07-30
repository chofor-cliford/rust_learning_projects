fn process_numbers(numbers: &[i32]) {
    // Initialize sum to 0
    let mut sum = 0;

    // Iterate over the numbers
    for number in numbers {
        // Add the number to the sum
        sum += number;
    }

    // Print the sum
    println!("The sum of the numbers is: {}", sum);

    // If the sum is even, print a message
    if sum % 2 == 0 {
        println!("The sum is even");
    } else {
        println!("The sum is odd");
    }

}

fn main() {
    process_numbers(&[1, 2, 3, 4, 5]);
}
