#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn width(&self) -> bool {
        self.width > 0
    }
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    println!("rect1 is {:?}", rect1);
    println!("rect1 is {:#?}", rect1);

    dbg!(&rect1);

    println!(
        "The area of the rectangle is {} square pixels (using function).",
        area(&rect1)
    );

    println!(
        "The area of the rectangle is {} square pixels (using method).",
        rect1.area()
    );

    if rect1.width() {
        println!("The rectangle has a non zero width; it is: {}", rect1.width);
    }
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}
