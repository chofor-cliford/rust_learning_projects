fn main() {

    // String slice
   let hello : &str = "Hello, World!";
    println!("String slice: {}", hello);

    // Creating an empty vector and adding elements to it
    let mut numbers: Vec<i32> = Vec::new();
    numbers.push(1);
    numbers.push(2);
    numbers.push(3);
    println!("Vector: {:?}", numbers);

    // Creating a string with from() method
    let mut greeting = String::from("Hello");
    greeting.push_str(", World!");
    println!("String: {}", greeting);
}
