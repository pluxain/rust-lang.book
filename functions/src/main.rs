fn main() {
    println!("Hello, world!");

    another_function(5);

    print_labeled_measurement(5, 'h');

    blocks_are_expressions();
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn blocks_are_expressions() {
    let y = {
        let x = 3;
        x + 1 // !! the missing ; is what make this an expression and not a statement
    };

    println!("The value of y is: {y}");
}
