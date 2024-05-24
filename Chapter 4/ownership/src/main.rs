fn main() {
    let s = "hello world";
    let mut string = String::from("Hello");
    string.push_str("World!");

    println!("s variable: {s}");
    println!("string variable: {s}");

    // Variables and Data Interacting with Move
    println!("\n\nVariables and Data Interacting with Move");
    let x = 5;
    let y = x;

    let s1 = String::from("hello");
    let _s2 = s1;

    // println!("{},world", s1);

    // Variables and Data Interacting with Clone
    println!("\n\nVariables and Data Interacting with Clone");
    let s1 = String::from("Hello");
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");

    // Stack-Only Data: Copy
    println!("\n\nStack-Only Data: Copy");
    println!("x = {}, y = {}", x, y);

    //Ownership and Functions
    println!("\n\nOwnership and Functions");

    let ownership = String::from("Hello");
    takes_ownership(ownership);
    // println!("{ownership}");

    let x = 5;
    makes_copy(x);
    println!("Calling X value on main: {x}");

    //Return Values and Scope
    println!("\n\nReturn Values and Scope");

    let ownership = gives_ownership();
    let ownership_s2 = String::from("hello");
    let ownership_s3 = takes_and_gives_back(ownership_s2);

    println!("\n\nInside main");
    println!("Ownership: {ownership}");
    // println!("Ownership_s2: {ownership_s2}"); //Trigers Borrow error
    println!("Ownership_s3: {ownership_s3}");

    let (s2, len) = calculate_length(ownership);

    println!("The length of '{}' is {}.", s2, len);
}

fn takes_ownership(string: String) {
    println!("{string}");
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}
