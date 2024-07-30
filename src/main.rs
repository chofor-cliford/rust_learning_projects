struct Person {
    first_name: String,
    last_name: String,
    age: Option<u8>,
}


fn main() {
let chidima = Person {
    first_name: "Chidima".to_string(),
    last_name: "Eze".to_string(),
    age: Some(100),
};

println!("First Name: {}", chidima.first_name);
println!("Last Name: {}", chidima.last_name);
println!("Age: {:?}", chidima.age);

// Or
// println!(":?", chidima);

}
