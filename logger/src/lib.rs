use ctor::ctor;
use log;
use log4rs;

#[ctor]
fn init() {
    log4rs::init_file("logger/logger.yaml", Default::default()).unwrap();
}

pub fn debug(m: &str) {
    log::debug!("{}", m);
}

pub fn error(m: &str) {
    log::error!("{}", m);
}

pub fn info(m: &str) {
    log::info!("{}", m);
}

pub fn warn(m: &str) {
    log::warn!("{}", m);
}
