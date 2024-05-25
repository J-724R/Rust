fn main() {
    // Refernce, now making the calculate example without borrowing
    println!("Now using references in calculate_length");

    let mut s1 = String::from("Hello");
    let len = calculate_length(&s1);
    println!("The length of '{s1}' is {}", len);

    // Mutable References
    println!("\n\nMutable References");
    change(&mut s1);
    println!("Mutated s1: {s1}");

    // Dangling References
    println!("\n\nDangling References");
    // let reference_to_nothing = dangle();
    let safe_return = no_dangle();
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world")
}

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}
