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

            let mut value = String::new();
            io::stdin()
                .read_line(&mut value)
                .expect("Failed to read line");

            let value: f32 = match value.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            println!(
                "I will convert {value}°{} into °{}",
                if direction == "1" { "C" } else { "F" },
                if direction == "1" { "F" } else { "C" }
            );
            break 'main;
        }
    }
}
