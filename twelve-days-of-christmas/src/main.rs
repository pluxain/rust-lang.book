fn main() {
    println!("# Twelve Days of Christmas");

    // Song pattern
    // On the first day of Christmas my true love sent to me
    // A partridge in a pear tree.

    // On the second day of Christmas my true love sent to me
    // Two turtle doves,
    // And a partridge in a pear tree.

    // On the third day of Christmas my true love sent to me
    // Three French hens,
    // Two turtle doves,
    // And a partridge in a pear tree.

    let ns = [
        ["first", "a partridge in a pear tree"],
        ["second", "two turtle doves"],
        ["third", "three French hens"],
        ["fourth", "four calling birds"],
        ["fifth", "five gold rings"],
        ["sixth", "six geese a-laying"],
        ["seventh", "seven swans a-swimming"],
        ["eighth", "eight maids a-milking"],
        ["ninth", "nine ladies dancing"],
        ["tenth", "ten lords a-leaping"],
        ["eleventh", "eleven pipers piping"],
        ["twelfth", "twelve drummers drumming"],
    ];

    for i in 0..ns.len() {
        println!();
        println!(
            "On the {} day of Christmas my true love sent to me",
            ns[i][0]
        );
        for j in 0..=i {
            println!("{}", capitalize(ns[j][1]));
        }
        println!();
    }
}

// @see https://stackoverflow.com/questions/38406793/why-is-capitalizing-the-first-letter-of-a-string-so-convoluted-in-rust/38406885?noredirect=1#comment65003459_38406885
fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
