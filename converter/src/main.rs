use std::io;

fn main() {
    println!("# Converter");
    loop {
        println!("What do you want to convert?");
        println!("1. Celsius to Fahrenheit (1)");
        println!("2. Fahrenheit to Celsius (2)");
        let mut direction = String::new();
        io::stdin()
            .read_line(&mut direction)
            .expect("Failed to read line");
        if !(["1", "2"].contains(&direction.trim())) {
            continue;
        }
        break;
    }
}
