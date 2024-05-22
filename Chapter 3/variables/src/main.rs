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

    // Chapter 3.3 Functions
    println!("Chapter 3.3 Functions");
    let score = ryuk(8, "medium");
    println!("Ryuk's score {score}");

    // Chapter 3.4 Comments
    println!("Chapter 3.4 Comments");

    // Chapter 3.5 Control Flow
    println!("Chapter 3.5 Control Flow");

    println!("Trying out loops");

    let mut counter = 0;
    loop {
        println!("Loop counter: {counter}");
        if counter == 10 {
            break;
        }
        counter += 1;
    }

    println!("\nLoop labels\n");

    'counting_up: loop {
        println!("Outer loop count = {counter}");
        let mut remaining = 10;

        loop {
            if counter == 2 {
                break 'counting_up;
            }
            println!("Inner loop remaining = {remaining}");
            if remaining == 1 {
                break;
            }
            remaining -= 1;
        }
        counter -= 1;
        if counter == 0 {
            println!("Unreachable code");
        }
    }
    println!("End counter = {counter}");

    println!("\nWhile loops\n");

    let mut number = 3;

    while number != 0 {
        println!("{number}!");
        number -= 1;
    }

    println!("LIFTOFF!!!");

    println!("\nFor loops\n");
    let a = ["Ryuk", "Luna", "Negro", "Neron", "Manchas", "Traviesa"];

    println!("Dogs names");
    for dog_name in a {
        println!("{dog_name}")
    }

    println!("\nFor loops with ranges\n");
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

fn ryuk(age: i8, size: &str) -> f32 {
    println!("Ryuk is the best dog!!!");
    println!("age: {age}, size: {size}");

    let score: f32 = 10.0;
    score
}
