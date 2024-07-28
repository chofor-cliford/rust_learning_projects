fn average_price(prices: &[f64]) -> f64 {
    let total: f64 = prices.iter().sum();
    total / prices.len() as f64
}

fn main() {
    println!("Hello, world!");
    println!("I'm a Rustacean!");

    // Variables
    let x = 5;
    let y = 10;
    let z = x + y;
    println!("The value of z is: {}", z);
}

