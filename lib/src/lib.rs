//! This is a library that provides a utility function to read a line from the standard input. 
//! So far it only provides a function to read a line from the standard input and return it as a String.
//! # Examples
//! ```
//! let input = lib::read_stdin();
//! println!("{}", input);
//! ```
//! 
//! # Panics
//! The function will panic if it fails to read a line from the standard input.

use std::io::{BufReader, BufRead};

/// This function reads a line from the standard input and returns it as a String.
/// It will panic if it fails to read a line.
///  # Examples
/// ```
/// let input = lib::read_stdin();
/// println!("{}", input);
/// ```
pub fn read_stdin() -> String {
    let stdin = std::io::stdin();
    let mut reader = BufReader::new(stdin.lock());
    let mut line = String::new();
    reader.read_line(&mut line).expect("Failed to read line");
    line.trim().to_string()
}

