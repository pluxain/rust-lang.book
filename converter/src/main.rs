use std::io;

fn main() {
    println!("# Converter");
    let units = [["Celsius", "C", "F"], ["Fahrenheit", "F", "C"]];
    let formulas = [|i: f32| i * 1.8 + 32.0, |i: f32| (i - 32.0) / 1.8];

    'main: loop {
        println!("What do you want to convert?");
        println!("1. {} to {} (1)", units[0][0], units[1][0]);
        println!("2. {} to {} (2)", units[1][0], units[0][0]);

        let mut input_type = String::new();
        io::stdin()
            .read_line(&mut input_type)
            .expect("Failed to read line");

        let input_type: usize = match input_type.trim().parse::<usize>() {
            Ok(num) if num <= units.len() => num - 1, // Note: we add a condition in this Arm to have the index from input_type directly
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

        loop {
            println!("Please enter a {} temperature:", units[input_type][0]);

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            let input: f32 = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            let output = formulas[input_type](input);

            println!(
                "{input}°{} converts to {output}°{}",
                units[input_type][1], units[input_type][2],
            );

            break 'main;
        }
    }
}
