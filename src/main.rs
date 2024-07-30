fn print_str(s: &str) {
    let mut new_string = format!("{}! other string", s);
    new_string.push_str("!");
    println!("{}", new_string);
}

fn print_string(s: String) {
    println!("{}", s);
}

fn main() {
    let s: &str = "hello, world";
    print_str(s);

    // String is a heap-allocated string
    // It is growable, mutable, and owned
    let salutation = String::from("hello");
    print_string(salutation);
}
