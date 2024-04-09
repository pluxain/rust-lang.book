use std::io; // introduce `io` from the standard library as it is not part of the prelude.

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    let mut guess = String::new(); // `let` introduces a variable, immutable by default. `mut` makes the variable mutable

    io::stdin()
        .read_line(&mut guess) // read line from io into a mutable reference `&mut var`
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
