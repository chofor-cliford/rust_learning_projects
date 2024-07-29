fn main() {
    let mut height = 190;
    height = height - 20;
    let result = if height > 180 {
        "Tall"
    } else if height > 170 {
        "Average"
    } else {
        "Short"
    };

    println!("The person is {}", result);

    let health = if height < 180 {"good"} else {"unknown"};
    println!("The person's health is {}", health);

    // shadowing a different type
    let health = if height < 180 {true} else {false};
    println!("Is the person healthy? {}", health);
}
