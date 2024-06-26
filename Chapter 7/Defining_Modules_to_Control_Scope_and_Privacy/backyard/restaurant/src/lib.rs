mod front_of_house;
// {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//         fn seat_at_table() {}
//     }

//     mod serving {
//         fn tke_order() {}
//         fn serve_order() {}
//         fn take_payment() {}
//     }
// }
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // Absolute path
    // crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    // front_of_house::hosting::add_to_waitlist();

    hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

fn deliver_order() {}

mod back_of_house;
// {
//     fn fix_incorrect_order() {
//         cook_order();
//         super::deliver_order();
//     }

//     pub struct Breakfast {
//         pub toast: String,
//         seasonal_fruit: String,
//     }

//     pub enum Appetizer {
//         Soup,
//         Salad,
//     }

//     impl Breakfast {
//         pub fn summer(toast: &strr) -> Breakfast {
//             Breakfast {
//                 toast: String::from(toastL),
//                 seasonal_fruit: String::from("peaches"),
//             }
//         }
//     }

//     fn cook_order() {}
// }

// Chapter 7.4 use keyword

use crate::front_of_house::hosting;

mod customer {
    pub fn eat_at_restaurant2() {
        super::hosting;
        hosting::add_to_waitlist();
    }
}

// pub fn eat_at_restaurant2() {
//     hosting::add_to_waitlist();
// }

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
