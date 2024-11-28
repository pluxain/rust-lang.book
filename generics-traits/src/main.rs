use log;
use log4rs;

#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
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

    log::info!("Generic Data Types");
    log::info!("In Function Definitions");

    let numbers = vec![34, 50, 25, 100, 65];
    let result = largest(&numbers);

    log::info!("The largest number from {:?} is {}", numbers, result);

    let chars = vec!['y', 'm', 'a', 'q'];
    let result = largest(&chars);
    log::info!("The largest chars from {:?} is {}", chars, result);

    log::info!("In Struct Definitions");
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    let mixed = Point { x: 5, y: 4.0 };
    log::debug!(
        "An integer Point {:?} with x being {} and y being {}",
        integer,
        integer.x,
        integer.y
    );
    log::debug!(
        "A float Point {:?} with x being {} and y being {}",
        float,
        float.x,
        float.y
    );
    log::debug!(
        "A mixed Point {:?} with x being {} and y being {}",
        mixed,
        mixed.x,
        mixed.y
    );
}
