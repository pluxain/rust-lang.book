use log;
use log4rs;

fn main() {
    log4rs::init_file("config/logger.yaml", Default::default()).unwrap();
    log::info!("Strings");

    let s = String::new();
    log::info!("{:?}", s);

    let data = "initial contents";
    let s = data.to_string();
    log::info!("{:?}", s);
    let s = "initial contents".to_string();
    log::info!("{:?}", s);
    let s = String::from("initial contents");
    log::info!("{:?}", s);
    let hellos = [
        "السلام عليكم",
        "Dobrý den",
        "Hello",
        "שלום",
        "नमस्ते",
        "こんにちは",
        "안녕하세요",
        "你好",
        "Olá",
        "Здравствуйте",
        "Hola",
    ];
    for s in hellos {
        let hello = String::from(s);
        log::info!("{:?}", hello);
    }
}
