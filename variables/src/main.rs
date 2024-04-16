// Constants can be declared in global scope
const THREE_HOURS_IN_SECONDS: u32 = 3 * 60 * 60;
fn main() {
    const UNITS_SECOND: &str = "sec"; // This is not in the book, but I wanted to know how to create a string variable
    println!("Three hours in seconds is: {THREE_HOURS_IN_SECONDS}{UNITS_SECOND}");

    const SIX_HOURS_IN_SECONDS: u32 = THREE_HOURS_IN_SECONDS * 2;
    println!("Six hours in seconds is: {SIX_HOURS_IN_SECONDS}{UNITS_SECOND}");

    let mut x = 5;
    println!("The value of x is : {x}");

    x = 6;
    println!("The value of x is : {x}");

    // Shadowing
    let y = 5;

    let y = y + 1;

    {
        // This marks a block with its own scope
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }
    println!("The value of y is: {y}");
}
