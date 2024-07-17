fn main() {
    // refutable pattern where Rust requires an irrefutable pattern
    // and vice versa

    // Will throw an error
    let Some(x) = some_option_value;

    // fixed
    if let Some(x) = some_option_value {
        println!("{x}");
    }

    // will throw warning
    if let x = 5 {
        println!("{x}");
    };
}
