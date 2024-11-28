use log;
use log4rs;

fn main() {
    log4rs::init_file("config/logger.yaml", Default::default()).unwrap();
    log::info!("Generics and Traits");

    log::info!("Removing Duplication by Extracting a Function");

    let numbers = vec![34, 50, 25, 100, 65];
    let mut largest = &numbers[0];

    for number in &numbers {
        if number > largest {
            largest = number;
        }
    }

    log::info!("The largest number from {:?} is {}", numbers, largest);

    let numbers = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let mut largest = &numbers[0];

    for number in &numbers {
        if number > largest {
            largest = number;
        }
    }

    log::info!("The largest number from {:?} is {}", numbers, largest);
}
