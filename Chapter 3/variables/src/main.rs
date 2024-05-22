fn main() {
    // Chapter 3.1. Variables and Mutability
    let mut x = 5;
    println!("The values of x is: {x}");
    x = 6;
    println!("The value of x is: {x}\n");

    // Shadowing
    println!("Shadowing");
    let y = 15;
    let y = y + 1;
    {
        let y = x + y;
        println!("The value of y in the inner scope is: {y}");
    }
    println!("The value of y is: {y}");

    // Chapter 3.2 Data Types
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of x, y and z are: {x} , {y} , {z}");

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    println!(
        "
        List  of values:
        five hundred : {five_hundred}
        six point four : {six_point_four}
        one : {one} 
    "
    );
}
