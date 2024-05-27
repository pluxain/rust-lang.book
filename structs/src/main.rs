struct User {
    active: bool,
    email: String,
    sign_in_count: u64,
    username: String,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

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

    let user2 = build_user(String::from("test@example.com"), String::from("player one"));
    print_user(&user2);

    // Struct update syntax
    let user3 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    print_user(&user3);
    // print_user(&user1); // That would not work because user1 was partially moved in user3
    user1.username = String::from("Jean-Claude");
    print_user(&user1); // Now it works and user3 stays as it was
    print_user(&user3);

    // Tuple struct
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    print_color("black", black);
    // print_color("not working", origin); // Won't work as Point is not Color even though they have the same structure.
    print_point("origin", origin);
}

fn print_color(colorname: &str, c: Color) {
    println!("{} is {} {} {}", colorname, c.0, c.1, c.2);
}

fn print_point(pointname: &str, p: Point) {
    println!("{} is {} {} {}", pointname, p.0, p.1, p.2);
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

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        email,
        sign_in_count: 1,
        username,
    }
}
