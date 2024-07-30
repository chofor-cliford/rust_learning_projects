fn ownership() {
    let numbers = vec![1, 2, 3, 4, 5];
    let slice = &numbers[..]; // slice of the entire vector
    println!("slice: {:?}", slice);
}

fn modifiable() {
    let mut numbers = vec![1, 2, 3, 4, 5];
    let slice = &mut numbers[..]; // mutable slice of the entire vector
    slice[0] = 10;
    // this would fail
    let other_slice = &numbers[..];
    println!("slice: {:?}", other_slice);
}

fn main() {
    // Slice and vectors are smilar. But slices are immutable depending on how they are borrowed.
    // ownership();
    modifiable();
}