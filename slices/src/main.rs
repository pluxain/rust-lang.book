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

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
