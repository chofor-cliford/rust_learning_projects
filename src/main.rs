fn main() {
   // let mut be some value or None
   let maybe_number : Option<Option <()>> = Some(None);
    // match maybe_number {
    //     Some(number) => println!("The number is: {}", number),
    //     None => println!("There is no number!"),
    // }

    if let Some(number) = maybe_number {
        println!("The number is: {:?}", number);
    } else {
        println!("There is no number!");
    }
}
