struct User {
    active: bool,
    email: String,
    sign_in_count: u64,
    username: String,
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername75"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    print_user(&user1);
    user1.sign_in_count = 2;
    user1.active = false;
    print_user(&user1);
}

fn print_user(u: &User) {
    println!(
        "{}<{}> is {} and signed in {} time(s)",
        u.username,
        u.email,
        if u.active { "active" } else { "not active" },
        u.sign_in_count
    );
}
