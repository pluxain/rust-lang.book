mod back_of_house;
mod front_of_house;

// Note: this is the normal idiom when specifying a path to use a function
// the parent module is imported rather than the function directly
// so that when we use it we know precisely where the function comes from
pub use crate::front_of_house::hosting;

pub mod customer {
    use super::hosting;
    // Or directly declare the use here
    // use crate::front_of_house::hosting;

    pub fn eat_at_restaurant() {
        // Absolut path -> preferred
        hosting::add_to_waitlist();

        // Relative path
        // front_of_house::hosting::add_to_waitlist();

        hosting::seat_at_table();

        crate::front_of_house::serving::take_order();
        // Order a breakfast in the Summer with Rye toast
        let mut meal = crate::back_of_house::Breakfast::summer("Rye");
        // Change our mind about what bread we'd like
        meal.toast = String::from("Wheat");
        println!("I'd like a {} toast please", meal.toast);
        // The next line won't compile if we uncomment it; we're not allowed
        // to see or modify the seasonal fruit that comes with the meal
        // meal.seasonal_fruit = String::from("blueberries");
        let _order1 = crate::back_of_house::Appetizer::Soup;
        let _order2 = crate::back_of_house::Appetizer::Salad;

        crate::front_of_house::serving::serve_order();
        crate::front_of_house::serving::take_payment();
    }
}

pub fn fix_incorrect_order() {
    crate::back_of_house::fix_incorrect_order();
}

fn deliver_order() {
    println!("Deliver order!");
}
