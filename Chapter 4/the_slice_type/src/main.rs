fn main() {
    let string_literal = "Hello, world";
    let my_string = String::from("hello world");

    let word = first_word(&my_string[0..6]);
    let word1 = first_word(&my_string[..]);

    println!("My string \n {word}, {word1}");

    let word = first_word(&string_literal[0..6]);
    let word1 = first_word(&string_literal[..]);

    println!("\nMy string literal \n {word}, {word1}");

    // String slices

    //String literals as slices

    //Other slices

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    // s.len();
    // String slices
    &s[..]
}
