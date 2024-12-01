use std::fmt::{Debug, Display};

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn notify(item: &impl Summary) {
    log::info!("Breaking news! {}", item.summarize());
}

// Using the Trait as parameter enables both parameter to only implement the Trait but not to be of the same Type
pub fn mixed_feed(item1: &impl Summary, item2: &impl Summary) {
    log::info!("\n> {} \n> {}", item1.summarize(), item2.summarize());
}

// Using the Trait bound syntax, both parameters have to be of the same Type
pub fn feed<T: Summary>(item1: &T, item2: &T) {
    log::info!("\n> {} \n> {}", item1.summarize(), item2.summarize());
}

// Using more than one Trait as parameter
pub fn with_more_traits_as_parameter(item: &(impl Summary + Display)) {
    log::debug!("{} {}", item.summarize(), item);
}

// Using more than one Trait as bound
pub fn with_more_traits_as_bound<T: Summary + PartialOrd>(item1: &T, item2: &T) {
    log::debug!(
        "{} >= {} ? {}",
        item1.summarize(),
        item2.summarize(),
        item1 >= item2
    );
}

pub fn some_function_with_obscure_types<T: Display + Clone, U: Clone + Debug>(
    _t: &T,
    _u: &U,
) -> i32 {
    0
}

pub fn some_function_using_where_to_increase_readability<T, U>(_t: &T, _u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    0
}

pub fn returns_summarizable() -> impl Summary {
    Tweet {
        username: "horse_ebooks".to_string(),
        content: "of course, as you probably already know, people".to_string(),
        reply: false,
        retweet: false,
    }
}

// Note: won't work because returned types are different
// fn returns_summarizable_wrong(switch: bool) -> impl Summary {
//     if switch {
//         NewsArticle {
//             headline: String::from("Penguins win the Stanley Cup Championship!"),
//             location: String::from("Pittsburgh, PA, USA"),
//             author: String::from("Iceburgh"),
//             content: String::from(
//                 "The Pittsburgh Penguins once again are the best \
//                  hockey team in the NHL.",
//             ),
//         }
//     } else {
//         Tweet {
//             username: String::from("horse_ebooks"),
//             content: String::from("of course, as you probably already know, people"),
//             reply: false,
//             retweet: false,
//         }
//     }
// }

pub struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn x(&self) -> &T {
        &self.x
    }

    pub fn y(&self) -> &T {
        &self.y
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    pub fn cmp_display(&self) {
        if self.x >= self.y {
            log::info!("The largest member is x = {}", self.x);
        } else {
            log::info!("The largest member is y = {}", self.y);
        }
    }
}

impl<T: Summary> ToString for T {
    fn to_string(&self) -> String {
        self.summarize()
    }
}
