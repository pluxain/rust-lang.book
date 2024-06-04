enum IpAddrKind {
    V4,
    V6,
}

fn main() {
    println!("Enums");
    let _four = IpAddrKind::V4;
    let _six = IpAddrKind::V6;

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
}

fn route(_ip_kind: IpAddrKind) {}
