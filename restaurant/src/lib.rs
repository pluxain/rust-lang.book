mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("Add to waitlist!");
        }

        pub fn seat_at_table() {
            println!("Seat at table!");
        }
    }

    pub mod serving {
        pub fn take_order() {
            println!("Take order!");
        }

        pub fn serve_order() {
            println!("Serve order!");
        }

        pub fn take_payment() {
            println!("Take payment!");
        }
    }
}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    // note that because back_of_house::Breakfast has a private field,
    // the struct needs to provide a public associated function
    // that constructs an instance of Breakfast
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub fn fix_incorrect_order() {
        println!("Fix incorrect order!");
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {
        println!("Cook order!");
    }
}

pub fn eat_at_restaurant() {
    // Absolut path -> preferred
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    // front_of_house::hosting::add_to_waitlist();

    crate::front_of_house::hosting::seat_at_table();

    crate::front_of_house::serving::take_order();
    // Order a breakfast in the Summer with Rye toast
    let mut meal = crate::back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like a {} toast please", meal.toast);
    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    crate::front_of_house::serving::serve_order();
    crate::front_of_house::serving::take_payment();
}

pub fn fix_incorrect_order() {
    crate::back_of_house::fix_incorrect_order();
}

fn deliver_order() {
    println!("Deliver order!");
}
