#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn width(&self) -> bool {
        self.width > 0
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("area of rect1 = {}", rect1.area());

    if rect1.width() {
        println!("{} is > 0", rect1.width);
    } else {
        println!("{} is <= 0", rect1.width);
    }

    println!(
        "can rect1 hold rect2? {}",
        if rect1.can_hold(&rect2) { "yah" } else { "nah" }
    );

    let square = Rectangle::square(10);
}
