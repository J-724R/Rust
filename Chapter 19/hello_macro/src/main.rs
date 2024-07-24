// #![allow(unused)]
// fn main() {
//     let v: Vec<u32> = vec![1, 2, 3];
// }

// #[macro_export]
// macro_rules! vec {
//     ( $( $x:expr ),* ) => {
//         {
//             let mut temp_vec = Vec::new();
//             $(
//                 temp_vec.push($x);
//             )*
//             temp_vec
//         }
//     };
// }

//

// -- Custom derive Macro --

use hello_macro::HelloMacro;

struct Pancakes;

impl HelloMacro for Pancakes {
    fn hello_macro() {
        println!("Hello, Macro! My name is Pancakes!");
    }
}

fn main() {
    Pancakes::hello_macro();
}
