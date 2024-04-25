use std::io;

fn main() {
    println!("# Converter");
    'main: loop {
        println!("What do you want to convert?");
        println!("1. Celsius to Fahrenheit (1)");
        println!("2. Fahrenheit to Celsius (2)");
        let mut direction = String::new();
        io::stdin()
            .read_line(&mut direction)
            .expect("Failed to read line");

        let direction = direction.trim();
        if !(["1", "2"].contains(&direction)) {
            continue;
        }

        loop {
            println!(
                "Please enter a {} temperature:",
                if direction == "1" {
                    "Celsius"
                } else {
                    "Fahrenheit"
                }
            );

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            let input: f32 = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            let output = if direction == "1" {
                input * 1.8 + 32.0
            } else {
                (input - 32.0) / 1.8
            };

            println!(
                "{input}°{} converts to {output}°{}",
                if direction == "1" { "C" } else { "F" },
                if direction == "1" { "F" } else { "C" }
            );

            break 'main;
        }
    }
}
