use log;
use log4rs;

fn main() {
    log4rs::init_file("config/logger.yaml", Default::default()).unwrap();
    log::info!("Vectors");

    log::info!("Creating a Vector");
    let v: Vec<i32> = Vec::new();
    log::info!("Empty Vector: {:?}", v);

    let v = vec![1, 2, 3];
    log::info!("Initialised Vector: {:?}", v);

    log::info!("Updating a Vector");
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    log::info!("Mutated Vector: {:?}", v);
}
