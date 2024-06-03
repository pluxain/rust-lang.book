use log;
use log4rs;

fn main() {
    log4rs::init_file("config/logger.yaml", Default::default()).unwrap();
    println!("Hello, world!");
    log::debug!("Hello world in debug mode!");
    log::info!("Hello world in info mode!");
    log::warn!("Hello world in warn mode!");
    log::error!("Hello world in error mode!");
}
