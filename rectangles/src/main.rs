#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.height > other.height && self.width > other.width
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

    let rect2 = Rectangle {
        height: 40,
        width: 10,
    };

    let rect3 = Rectangle {
        height: 45,
        width: 60,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}
