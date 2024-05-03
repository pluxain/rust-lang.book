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
