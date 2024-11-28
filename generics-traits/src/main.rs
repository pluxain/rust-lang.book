use log;
use log4rs;

fn largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    log4rs::init_file("config/logger.yaml", Default::default()).unwrap();
    log::info!("Generics and Traits");

    log::info!("Removing Duplication by Extracting a Function");

    let numbers = vec![34, 50, 25, 100, 65];
    let result = largest(&numbers);

    log::info!("The largest number from {:?} is {}", numbers, result);

    let numbers = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result = largest(&numbers);
    log::info!("The largest number from {:?} is {}", numbers, result);
}
