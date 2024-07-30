#[derive(Debug)]
struct User {
    username: String,
    email: String,
    uri: String,
    active: bool,
}

impl User {
    fn new(username: String, email: String, uri: String) -> Self {
        Self {
            username,
            email,
            uri,
            active: true,
        }
    }

    fn from_email(email: String) -> Self {
        let username = email.split('@').next().unwrap().to_string();
        Self {
            username,
            email,
            uri: String::new(),
            active: true,
        }
    }

    fn deactivate(&mut self) {
        self.active = false;
    }

    fn update_uri(&mut self, new_uri: String) {
        self.uri = new_uri;
    }
}

fn main() {
    let mut new_user = User::new(
        String::from("alfredodeza"),
        String::from("alfreodeza@example.com"),
        String::from("https://alfredodeza.com"),
    );
    println!("Hello, {}!", new_user.username);
    println!("Account {} status is: {}", new_user.username, new_user.active);
    new_user.deactivate();
    println!("Account {} status is: {}", new_user.username, new_user.active);

    let mut user_from_email = User::from_email(String::from("john@example.com"));
    println!("{:?}", user_from_email);

    user_from_email.update_uri(String::from("https://johnexample.com"));
    println!("Updated URI for {}: {}", user_from_email.username, user_from_email.uri);
}
