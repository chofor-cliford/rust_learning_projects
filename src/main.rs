// This example is a useful application of 'while' because it allows to continue
// reading lines from the standard input until the user types 'exit' or 'stop'.

use std::io;

fn main() {
    let mut input = lib::read_stdin();
    while input != "exit" && input != "stop" {
        println!("Please enter a word (type 'stop' or 'exit' to finish):");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        input = input.trim().to_string();
        println!("You entered: {}", input);
    }

    println!("Goodbye!");
}
