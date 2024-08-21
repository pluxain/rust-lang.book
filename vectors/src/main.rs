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

    log::info!("Read Elements of Vectors");
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    log::info!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(found) => log::info!("The third element is {found}"),
        None => log::info!("There is no third element."),
    }

    let v = vec![1, 2, 3, 4, 5];
    // let does_not_exist = &v[100]; // -> panicking
    // log::info!("Accessing a nonexistent value via index: {does_not_exist}");
    let does_not_exist = v.get(100);
    log::info!(
        "Accessing a nonexistent value via `get`: {:?}",
        does_not_exist
    );

    log::info!("Iterating over values in a Vector");
    let v = vec![100, 32, 57];
    for i in &v {
        log::info!("Iterating: {i}");
    }

    let mut v = vec![100, 32, 57];
    log::info!("Before iterating with mutable: {:?}", v);
    for i in &mut v {
        *i += 50;
    }
    log::info!("After iterating with mutable: {:?}", v);

    log::info!("Using an Enum to store multiple Types");
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    log::info!("row is {:?}", row);
    for i in &row {
        match i {
            SpreadsheetCell::Int(int) => log::info!("i is an Int: {int}"),
            SpreadsheetCell::Float(float) => log::info!("i is a Float: {float}"),
            SpreadsheetCell::Text(text) => log::info!("i is a Text: {text}"),
        }
    }
}
