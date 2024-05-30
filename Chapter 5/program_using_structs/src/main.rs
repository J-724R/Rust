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
    let scale = 2;
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: dbg!(10 * scale),
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    let sq = Rectangle::square(3);

    println!(
        "The area of the reactangle is {} aquare pixels.",
        area(&rect1)
    );

    println!("rect1 is {:#?}", rect1);
    dbg!(&rect2);

    println!("\nUsing methods syntax");

    println!(
        "The area of the reactangle is {} aquare pixels.",
        rect1.area()
    );

    println!("The rectangle has a nonero width; it is {}", rect1.width());

    println!("Methods with More Parameters");

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    println!("Associated Functions");
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
