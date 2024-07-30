fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    v.push(6);
    // println!("{:?}", v); // [1, 2, 3, 4, 5, 6]

    // extend adds each element of the given slice to the vector
    let more_numbers = vec![7, 8, 9];
    v.extend(more_numbers);
    // println!("{:?}", v); // error: value borrowed here after move

    // append adds the given vector to the vector, requires the vector to be mutable
    let mut other_numbers = vec![1, 2, 3, 4, 5];
    v.append(&mut other_numbers);
    // println!("{:?}", v); // [1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5]

    // insert items at a given index
    v.insert(2, 10);
    println!("{:?}", v); // [1, 2, 10, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5]
}