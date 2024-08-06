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

pub fn eat_at_restaurant() {
    // Absolut path -> preferred
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    // front_of_house::hosting::add_to_waitlist();

    crate::front_of_house::hosting::seat_at_table();

    crate::front_of_house::serving::take_order();
    crate::front_of_house::serving::serve_order();
    crate::front_of_house::serving::take_payment();
}
