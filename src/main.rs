#[derive(Debug)]
struct User {
    username: String,
    email: String,
    uri: String,
    active: bool,
}

struct  Points (i32, i32, i32);

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
    let username = String::from("alfredodeza");
    let email = String::from("alfreodeza@example.com");
    let uri = String::from("https://alfredodeza.com");
    let active = true;

    let mut new_user = User{
        username,
        email,
        uri,
        active,
    };

    let points = Points(1, 2, 3);
    println!("Points: {}, {}, {}", points.0, points.1, points.2);

    println!("Hello, {}!", new_user.username);
    println!("Account {} status is: {}", new_user.username, new_user.active);
    new_user.deactivate();
    println!("Account {} status is: {}", new_user.username, new_user.active);

    let mut user_from_email = User::from_email(String::from("john@example.com"));
    println!("{:?}", user_from_email);

    user_from_email.update_uri(String::from("https://johnexample.com"));
    println!("Updated URI for {}: {}", user_from_email.username, user_from_email.uri);
}
