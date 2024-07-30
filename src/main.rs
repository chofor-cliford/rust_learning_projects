fn split_string(s: String, delimiter: char, field: usize) -> String {
    let parts: Vec<&str> = s.split(delimiter).collect();
    let result = parts.get(field);

    // This would not compile!
    result.expect("oops! something went wrong").to_string() // To panic should we get an error.
}

fn main() {
    let chunck = split_string("Hello, World".to_string(), ',', 1);
    println!("Split string: {}", chunck);
}
 // This is the genric unit function