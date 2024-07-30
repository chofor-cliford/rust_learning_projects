use std::io;

fn main() {
// let name = "Python";

// use match to check the value of name
// match name {
//     "Rust" => println!("I'm learning Rust"),
//     "Python" => println!("I'm learning Python"),
//     _ => println!("I'm learning something else"),
// }

    println!("Please enter a greeting:");

    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read line");

    // use of match to check the value of name
    match name.trim() {
        "Hello" => println!("Hello!"),
        "Hi" => println!("Hi, nice to meet you!"),
        "Hey" => println!("Hey!"),
        "Good Bye" => println!("Sorry to see you go!"),
        _ => println!("I don't know what you said"),
    }

}
