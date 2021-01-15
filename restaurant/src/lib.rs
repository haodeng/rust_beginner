
mod front_of_house {
    // Moodule must be pub for eat_at_restaurant to call
    pub mod hosting {
        // function must be pub for eat_at_restaurant
        pub fn add_to_waitlist() {}

        // By default is private
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        //  use super to go to the parent module of back_of_house, which in this case is crate, the root.
        super::serve_order();
    }

    fn cook_order() {}

    // struct can be pub too
    pub struct Breakfast {
        // We can make each field public or not on a case-by-case basis.
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // if we make an enum public, all of its variants are then public.
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

use crate::front_of_house::hosting;
// We can also bring an item into scope with use and a relative path.
//use self::front_of_house::hosting;

// Re-exporting Names with pub use, external code can now call the add_to_waitlist function using hosting::add_to_waitlist.
//pub use crate::front_of_house::hosting;

// bring the crate::front_of_house::hosting module into the scope of the eat_at_restaurant2 function
pub fn eat_at_restaurant2() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

