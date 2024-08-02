use std::io::{BufRead, BufReader};

pub fn read_stdin() -> String {
    let stdin = std::io::stdin();
    let mut reader = BufReader::new(stdin.lock());
    _read_stdin(&mut reader)
}

fn _read_stdin<R: BufRead> (reader: &mut R) -> String {
    let mut line = String::new();
    reader.read_line(&mut line).expect("Failed to read line");
    line.trim().to_string()
}

fn main() {
    let input = read_stdin();
    println!("{}", input);
}