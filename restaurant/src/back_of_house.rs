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

// Enums aren’t very useful unless their variants are public;
// it would be annoying to have to annotate all enum variants with pub in every case,
// so the default for enum variants is to be public.
pub enum Appetizer {
    Soup,
    Salad,
}

pub fn fix_incorrect_order() {
    println!("Fix incorrect order!");
    cook_order();
    super::deliver_order();
}

fn cook_order() {
    println!("Cook order!");
}
