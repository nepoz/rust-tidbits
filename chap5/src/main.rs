// Simple struct practice to define a Rectangle struct
#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.length
    }

    fn perimeter(&self) -> u32 {
        2 * (self.width + self.length)
    }

    fn can_hold(&self, r: &Rectangle) -> bool {
        r.width <= self.width && r.length < self.length
    }

    fn new(length: u32, width: u32) -> Rectangle {
        Rectangle {
            length,
            width,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        length: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        length: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        length: 45,
    };

    let rect4 = Rectangle::new(20, 20);

    println!("rect 1 has area {}", rect1.area());
    println!("rect1 has perimeter {}", rect1.perimeter());
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("Here's our new rectangle: {:#?}", rect4);
    println!("rect4 has area: {}", rect4.area());
}
