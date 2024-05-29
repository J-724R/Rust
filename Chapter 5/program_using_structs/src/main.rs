struct Rectangle {
    width: u32,
    height: u32,
}
fn main() {
    let react1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the reactangle is {} aquare pixels.",
        area(&react1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
