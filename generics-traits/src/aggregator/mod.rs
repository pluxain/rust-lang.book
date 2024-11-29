use std::fmt::Display;

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
