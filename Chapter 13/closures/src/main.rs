// use std::thread;

// #[derive(Debug, PartialEq, Copy, Clone)]
// enum ShirtColor {
//     Red,
//     Blue,
// }

// struct Inventory {
//     shirts: Vec<ShirtColor>,
// }

// impl Inventory {
//     fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
//         user_preference.unwrap_or_else(|| self.most_stocked())
//     }

//     fn most_stocked(&self) -> ShirtColor {
//         let mut num_red = 0;
//         let mut num_blue = 0;

//         for color in &self.shirts {
//             match color {
//                 ShirtColor::Red => num_red += 1,
//                 ShirtColor::Blue => num_blue += 1,
//             }
//         }
//         if num_red > num_blue {
//             ShirtColor::Red
//         } else {
//             ShirtColor::Blue
//         }
//     }
// }

// fn main() {
//     let store = Inventory {
//         shirts: vec![ShirtColor::Blue, ShirtColor::Blue, ShirtColor::Red],
//     };

//     let user_pref1 = Some(ShirtColor::Red);
//     let giveaway1 = store.giveaway(user_pref1);
//     println!(
//         "The user with preference {:?} gets {:?}",
//         user_pref1, giveaway1
//     );

//     let user_pref2 = None;
//     let giveaway2 = store.giveaway(user_pref2);
//     println!(
//         "The user with preference {:?} gets {:?}",
//         user_pref2, giveaway2
//     );

//     // Closure Type Inference and Annotation
//     // let expensive_closure = |num: u32| -> u32 {
//     //     println!("calculating slowly...");
//     //     thread::sleep(Duration::from_secs(2));
//     //     num
//     // };

//     // Comparision with Function
//     // fn add_one_v1(x: u32) -> u32 { x + 1  }
//     // let add_one_v2 = |x: u32| -> u32 { x + 1 };
//     // let add_one_v3 = |x| x + 1;
//     // let add_one_v4 = |x| x + 1;

//     // Capturing References or Moving Ownership
//     let list = vec![1, 2, 3];
//     println!("Before defining closure: {list:?}");

//     let only_borrows = || println!("From closure: {list:?}");

//     println!("Before calling closure: {list:?}");
//     only_borrows();
//     println!("After calling closure: {list:?}");
// }

// fn main() {
//     let list = vec![1, 2, 3];
//     println!("Before defining closure: {list:?}");

//     thread::spawn(move || println!("From thread: {list:?}"))
//         .join()
//         .unwrap();
// }

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    list.sort_by_key(|r| r.width);
    println!("{list:#?}");
}
