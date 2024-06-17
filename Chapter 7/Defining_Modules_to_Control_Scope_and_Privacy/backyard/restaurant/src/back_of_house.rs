mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }

    impl Breakfast {
        pub fn summer(toast: &strr) -> Breakfast {
            Breakfast {
                toast: String::from(toastL),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn cook_order() {}
}
