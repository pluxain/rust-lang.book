use log;
use log4rs;

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item
        }
    }
    largest
}

fn main() {
    log4rs::init_file("config/logger.yaml", Default::default()).unwrap();
    log::info!("Generics and Traits");

    log::info!("Removing Duplication by Extracting a Function");

    let numbers = vec![34, 50, 25, 100, 65];
    let result = largest_i32(&numbers);

    log::info!("The largest number from {:?} is {}", numbers, result);

    let numbers = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result = largest_i32(&numbers);
    log::info!("The largest number from {:?} is {}", numbers, result);

    log::info!("Generic Data Types");
    log::info!("In Function Definitions");

    let numbers = vec![34, 50, 25, 100, 65];
    let result = largest_i32(&numbers);

    log::info!("The largest number from {:?} is {}", numbers, result);

    let chars = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(&chars);
    log::info!("The largest chars from {:?} is {}", chars, result);
}
