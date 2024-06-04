use log;
use log4rs;

#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}
fn main() {
    log4rs::init_file("config/logger.yaml", Default::default()).unwrap();
    log::info!("Enums");

    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    log::debug!("{:?}", home);
    log::debug!("{:?}", loopback);
}
