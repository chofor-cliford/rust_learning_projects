fn main() {
for i in 1..=10 {
    if i %2 == 0 {
        // Skip even numbers
        continue;
    }
    println!("{}", i);
    if i == 7 {
        // Exit the loop early
        break;
    }
}
}
