use std::io::{self, stdin};

pub fn prompt(p: String) -> Result<String, io::Error> {
    log::info!("Prompting");
    let mut input = String::new();
    println!("{p}");
    match stdin().read_line(&mut input) {
        Ok(_) => Ok(input.trim().to_string()),
        Err(err) => Err(err),
    }
}
