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

    log::info!("Updating a String");
    log::info!("with methods `push_str` and `push`");
    let mut s = String::from("foo");
    let s2 = "bar";
    s.push_str(s2);
    log::info!("{:?}", s);
    log::info!("s2 is {s2}");

    let mut s = String::from("lo");
    s.push('l');
    log::info!("{:?}", s);

    log::info!("with `+` operator");
    let s1 = String::from("Hello, ");
    let mut s2 = String::from("world!");
    let s3 = s1 + &s2; // note: s1 has been moved here and can no longer be used;
    log::info!("s3 is {:?} and s2 is {:?}", s3, s2);
    s2.push_str(" Hahaha!");
    log::info!("s3 is {:?} and s2 is {:?}", s3, s2);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3;
    log::info!("{:?}", s);
    // log::info!("{:?}", s1); // -> fails as s1 was moved into s
    let s1 = String::from("tic");
    let s = format!("{s1}-{s2}-{s3}");
    log::info!("{:?}", s);
    log::info!(
        "s1 ({:?}) is still available as `format!` uses references",
        s1
    );

    log::info!("Slicing Strings");
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    log::info!("{:?}", s);
    // let s = &hello[0..1]; // -> panic as chars in the String are two bytes in this case.
    log::info!("{:?}", s);
    // log::info!("{:?}", s);

    log::info!("Methods for Iterating over Strings");
    let mut i = 0;
    for c in hello.chars() {
        log::info!("Char at {i} in {:?} is {:?}", hello, c);
        i += 1;
    }

    i = 0;
    for b in hello.bytes() {
        log::info!("Byte at {i} in {:?} is {:?}", hello, b);
        i += 1;
    }

    log::warn!("Strings Are Not So Simple");
}
