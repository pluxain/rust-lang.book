use std::io;

// Constants can be declared in global scope
const THREE_HOURS_IN_SECONDS: u32 = 3 * 60 * 60;
fn main() {
    println!("# CONSTANTS");
    const UNITS_SECOND: &str = "sec"; // This is not in the book, but I wanted to know how to create a string variable
    println!("Three hours in seconds is: {THREE_HOURS_IN_SECONDS}{UNITS_SECOND}");

    const SIX_HOURS_IN_SECONDS: u32 = THREE_HOURS_IN_SECONDS * 2;
    println!("Six hours in seconds is: {SIX_HOURS_IN_SECONDS}{UNITS_SECOND}");

    println!("# MUTABLE VARIABLES");
    let mut x = 5;
    println!("The value of x is : {x}");

    x = 6;
    println!("The value of x is : {x}");

    println!("# SHADOWING");
    let y = 5;

    let y = y + 1;

    {
        // This marks a block with its own scope
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }
    println!("The value of y is: {y}");

    println!("# SCALAR TYPES");
    println!("## INTEGER AND FLOATS");
    // addition
    let sum = 5 + 10;
    println!("5 + 10 is: {sum}");

    // subtraction
    let difference = 95.5 - 4.3;
    println!("95.5 - 4.3 is: {difference}");

    // multiplication
    let product = 4 * 30;
    println!("4 * 30 is: {product}");

    // division
    let quotient = 56.7 / 32.2;
    println!("56.7 / 32.2 is: {quotient}");
    let truncated = -5 / 3; // Results in -1
    println!("-5 / 3 is: {truncated}");

    // remainder
    let remainder = 43 % 5;
    println!("43 % 5 is: {remainder}");

    println!("## BOOLEANS");
    let t = true;
    println!("true is: {t}");

    let f: bool = false; // with explicit type annotation
    println!("false is: {f}");

    println!("## CHARS");
    let c = 'z'; // single quotes
    println!("z char is: {c}");
    let z: char = 'ℤ'; // with explicit type annotation
    println!("ℤ char is: {z}");
    let heart_eyed_cat = '😻';
    println!("😻 char is: {heart_eyed_cat}");

    println!("# COMPOUND TYPES");
    println!("## TUPLES");
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // println!("Tuple (i32, f64, u8) is : {tup}"); // -> fails! `(i32, f64, u8)` cannot be formatted with the default formatter
    let (x, y, z) = tup; // destructuring, without noticing we shadowed z char...
    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
    println!("The value of z is: {z}");

    let five_hundred = tup.0; // access tuple part by index
    let six_point_four = tup.1;
    let one = tup.2;
    println!("The value of five_hundred is: {five_hundred}");
    println!("The value of six_point_four is: {six_point_four}");
    println!("The value of one is: {one}");

    println!("## ARRAYS");
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let _s = [3; 5]; // initialize and array with same value

    // println!("The value of a is: {a}"); // -> fails! `[{integer}; 5]` cannot be formatted with the default formatter
    let _months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ]; // were know this is a fixed length list so array is a good choice.

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
