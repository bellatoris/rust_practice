#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn blahblah(&mut self, rect2: &mut Rectangle) -> u32 {
        (*self).area() + (*rect2).area()
    }
    
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    println!("rect1 is {:#?}", rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    let mut rect2 = Rectangle { ..rect1 };
    let mut rect3 = Rectangle { ..rect2 };
    println!(
        "The area of the rectangle is {} square pixels.",
        rect3.blahblah(&mut rect2)
    );

    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let rect = Rectangle::square(3);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

