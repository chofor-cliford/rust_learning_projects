fn divide (x: i32, y: i32) -> Option<i32> {
    if y == 0 {
        None
    } else {
        Some(x / y)
    }
}

fn main() {
    let x = 10;
    let y = 0;
    match divide(x, y) {
        Some(result) => println!("The result is {}", result),
        None => println!("Cannot divide by zero")
    }
}