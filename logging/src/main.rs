use logger;

fn main() {
    println!("Hello, world!");
    logger::debug("Hello world in logger debug mode!");
    logger::info("Hello world in logger info mode!");
    logger::warn("Hello world in logger warn mode!");
    logger::error("Hello world in logger error mode!");
}
