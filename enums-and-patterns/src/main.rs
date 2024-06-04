use log;
use log4rs;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

// #[derive(Debug)]
// enum IpAddr {
//     V4(u8, u8, u8, u8),
//     V6(String),
// }
fn main() {
    log4rs::init_file("config/logger.yaml", Default::default()).unwrap();
    log::info!("Enums");

    // let home = IpAddr::V4(127, 0, 0, 1);
    // let loopback = IpAddr::V6(String::from("::1"));
    let home = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let loopback = IpAddr::V6(Ipv6Addr::new(234, 2344, 324, 3445, 3435, 3435, 1, 55));

    log::debug!("{:?}", home);
    log::debug!("{:?}", loopback);
}
