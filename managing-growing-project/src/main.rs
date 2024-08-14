use log;
use log4rs;

fn main() {
    log4rs::init_file("config/logger.yaml", Default::default()).unwrap();
    log::info!("Managing growing Project");
}
