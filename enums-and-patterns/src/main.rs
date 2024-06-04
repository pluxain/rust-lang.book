use log;
use log4rs;

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    log4rs::init_file("config/logger.yaml", Default::default()).unwrap();
    log::info!("Enums");
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    log::debug!("{:?}", four);
    log::debug!("{:?}", six);

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    log::debug!("{:?}", home);
    log::debug!("{:?}", loopback);
}

fn route(ip_kind: IpAddrKind) {
    log::debug!("route: {:?}", ip_kind);
}
