fn main() {
    // Chapter 3.1. Variables and Mutability
    let mut x = 5;
    println!("The values of x is: {x}");
    x = 6;
    println!("The value of x is: {x}\n");

    // Shadowing

    let y = 15;
    let y = y + 1;
    {
        let y = x + y;
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y is: {y}");

    // Chapter 3.2 Data Types
}
