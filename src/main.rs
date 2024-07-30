use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if a file path was provided
    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];

    // Attempt to open the file
    let file = File::open(file_path)?;

    // Read the file line by line and print its contents
    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(content) => println!("{}", content),
            Err(e) => eprintln!("Error reading line: {}", e),
        }
    }

    Ok(())
}
