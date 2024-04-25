use std::io;

fn main() {
    println!("# Converter");
    let conversions: [(&str, &str, &str, fn(f32) -> f32); 2] = [
        ("Celsius", "C", "F", |i: f32| i * 1.8 + 32.0),
        ("Fahrenheit", "F", "C", |i: f32| (i - 32.0) / 1.8),
    ];

    'main: loop {
        println!("What do you want to convert?");
        println!("1. {} to {} (1)", conversions[0].0, conversions[1].0);
        println!("2. {} to {} (2)", conversions[1].0, conversions[0].0);

        let mut input_type = String::new();
        io::stdin()
            .read_line(&mut input_type)
            .expect("Failed to read line");

        let input_type: usize = match input_type.trim().parse::<usize>() {
            Ok(num) if num <= conversions.len() => num - 1, // Note: we add a condition in this Arm to have the index from input_type directly
            Ok(_num) => {
                // Note: this Arm comes into play if the parsing to usize worked, but the condition is not met
                // println!("Ok Arm condition failed");
                continue;
            }
            Err(_) => {
                // Note: could not parse to u8
                // println!("Err Arm");
                continue;
            }
        };

        let (i_name, i_unit, o_unit, formula) = conversions[input_type];

        loop {
            println!("Please enter a {i_name} temperature:");

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            let input: f32 = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            let output = formula(input);

            println!("{input}°{i_unit} converts to {output}°{o_unit}",);

            break 'main;
        }
    }
}
