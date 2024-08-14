// The normal idiom to import struc, enums and other items than function is
// to import them directly
use restaurant::{self, customer, hosting};
use std::collections::HashMap;
// We import only the parent modules of both Result types to avoid collision
// use std::{fmt::Result, io::Result as IoResult};

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
    hosting::add_to_waitlist();
    customer::eat_at_restaurant();
    restaurant::fix_incorrect_order();
}

// fn function1() -> Result {}
// fn function2() -> IoResult<()> {}
