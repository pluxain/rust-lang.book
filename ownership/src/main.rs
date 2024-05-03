fn main() {
    let str = "Hello";
    println!("{}", str);

    {
        let scoped_str = " world!";
        println!("{}{}", str, scoped_str);
    }
    // Fails as scoped_str does not exist anymore
    // println!("{}{}", str, scoped_str);

    let mut string = String::from("Hello");
    {
        let scrope_string = String::from(" world!");
        println!("{}{}", string, scrope_string);
    }
    string.push_str(" world!");
    println!("{}", string);

    // Ownership and function
    let ownership_string = String::from("Hello"); // ownership_string comes into scope
    println!("{}", ownership_string);

    takes_ownership(ownership_string); // ownership_string's value moves into the function...
                                       // and so is no longer available here

    // println!("{}", ownership_string); // fails because `takes_ownership` is now the owner of `ownership_string` and String is Drop
    let x = 5; // x comes into scope
    println!("{}", x);
    makes_copy(x); // x would move into the function,
                   // but i32 is Copy, so it's okay to still
                   // use x afterward

    println!("{}", x); // Works because i32 is Copy

    // Return values and Scope
    let s1 = gives_ownership(); // gives_ownership moves its return value into s1

    println!("{}", s1);

    let s2 = String::from("hello"); // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into takes_and_gives_back, which also  moves its return value into s3

    println!("{}", s3);

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
} // Here, x goes out of scope, then ownership_string. But because ownership_string's value was moved, nothing
  // special happens.

fn takes_ownership(s: String) {
    // s comes into scope
    println!("{}", s);
} // Here, s goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(i: i32) {
    // i comes into scope
    println!("{}", i);
} // Here, some_integer goes out of scope. Nothing special happens.

// gives_ownership will move its
// return value into the function
// that calls it
fn gives_ownership() -> String {
    let s = String::from("yours"); // s comes into scope
    s // some_string is returned and moves out to the calling function
}

// This function takes a String and returns one
// s comes into scope
fn takes_and_gives_back(s: String) -> String {
    s // s is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    let l = s.len();
    (s, l)
}
