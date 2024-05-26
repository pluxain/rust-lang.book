struct User {
    active: bool,
    email: String,
    sign_in_count: u64,
    username: String,
}

fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername75"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    println!(
        "{}<{}> is {} and signed in {} time(s)",
        user1.username,
        user1.email,
        if user1.active { "active" } else { "not active" },
        user1.sign_in_count
    );
}
