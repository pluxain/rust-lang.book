fn main() {
    let mut s = String::from("hello world");

    let _word = first_word_usize(&s);

    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!

    let s1 = String::from("hello Rust");
    let hello = &s1[..5]; // Rust Range syntax enables to omit the start value if it is 0
    let rust = &s1[6..]; // Rust Range syntax enables to omit the end value to get the last byte
    let entire = &s1[..]; // Equivalent to &s1[0..s1.len()]
    println!("{} {} | {}", hello, rust, entire);

    // UTF-8
    let utf8 = String::from("été");
    println!("length of {} is {}", utf8, utf8.len()); // utf8.len() is 5!

    s.push_str("hello world");
    let first = first_word(&s);
    // s.clear(); // error!
    println!("the first word is `{}`", first);

    // String literals Slices
    let s = "Hello Rust!";
    let first = first_word(s);
    println!("the first word is `{}`", first);

    let my_string = String::from("hello rust");

    // `first_word` works on Slices of String, whether partial or whole
    let _word = first_word(&my_string[..6]);
    let _word = first_word(&s[..]);
    // `first_word` works on reference to String, which are equivalent to whole slices of String
    let _word = first_word(&my_string);

    let my_string_literal = "hello rust";
    // `first_word` works on slices of string literals, whether partial or whole
    let _word = first_word(&my_string_literal[..6]);
    let _word = first_word(&my_string_literal[..]);

    // Because string literals are String Slices already,
    // this works too
    let _word = first_word(my_string_literal);

    // Other slices
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}

fn first_word_usize(s: &String) -> usize {
    let bytes = s.as_bytes(); // convert s in array of bytes

    // iter() creates an enumerator on bytes and enumerate returns a tuple with first element the index and second a reference to the value
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

//Defining a function to take a string slice instead of a reference to a String
// makes our API more general and useful without losing any functionality
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
