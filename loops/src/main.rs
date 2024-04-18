fn main() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    let mut n = 3;
    while n != 0 {
        println!("{n}!");
        n -= 1;
    }
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    println!("using `while` to loop through an array");
    // while index < a.len() {
    // Error prone as using hard coded number
    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    println!("using `for ... in` to loop through an array");
    for element in a {
        println!("the value is: {element}");
    }

    println!("countdown using `for` loop");
    for i in (1..4).rev() {
        println!("{i}!");
    }
    println!("LIFTOFF!!!");
}
