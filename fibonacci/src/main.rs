use std::io;

fn main() {
    println!("# Fibonacci number");

    loop {
        println!("Enter a positive integer please (q to exit):");
        let mut n = String::new();
        io::stdin()
            .read_line(&mut n) // read line from io into a mutable reference `&mut var`
            .expect("Failed to read line");

        if n.trim() == "q" {
            break;
        }

        let n: u32 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        for i in 0..=n {
            println!("Fibonacci number for {i} is {}", fibonacci(i));
        }
    }
}

fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 2) + fibonacci(n - 1),
    }
}
