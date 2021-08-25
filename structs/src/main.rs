#![allow(unused_variables)]
fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
    println!("user.email = {}", user1.email);

    let user1 = build_user("nahco3_hanmail.net".to_string(), "hahasiyoon".to_string());
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };
    println!("{}", print_user(user1));
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn print_user(user: User) -> String {
    format!(
        "user's name is {}, email is {}, active is {} and sign in count is {}",
        user.email, user.username, user.active, user.sign_in_count
    )
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
