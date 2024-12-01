use aggregator::{
    feed, first_word, mixed_feed, notify, returns_summarizable, ImportantExcerpt, NewsArticle,
    Pair, Summary, Tweet,
};
use log;
use log4rs;

pub mod aggregator;

#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &U {
        &self.y
    }
}

impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct MixPoint<X1, Y1> {
    x: X1,
    y: Y1,
}

// Note: The purpose of this example is to demonstrate a situation in which
// some generic parameters are declared with impl
// and some are declared with the method definition.
// Here, the generic parameters X1 and Y1 are declared after impl
// because they go with the struct definition.
// The generic parameters X2 and Y2 are declared after fn mixup
// because they’re only relevant to the method.
impl<X1, Y1> MixPoint<X1, Y1> {
    fn mixup<X2, Y2>(self, other: MixPoint<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// Note: here the lifetime elision cannot apply, because the compile cannot guess the input lifetimes and the output lifetimes:
// fn longest(x: &str, y: &str) -> &str {
// -> fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {
// here the compiler does not know what will be the output lifetime so it triggers an error
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    log4rs::init_file("config/logger.yaml", Default::default()).unwrap();
    log::info!("Generics and Traits");

    log::info!("Removing Duplication by Extracting a Function");

    let numbers = vec![34, 50, 25, 100, 65];
    let result = largest(&numbers);

    log::info!("The largest number from {:?} is {}", numbers, result);

    let numbers = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result = largest(&numbers);
    log::info!("The largest number from {:?} is {}", numbers, result);

    log::info!("Generic Data Types");
    log::info!("In Function Definitions");

    let numbers = vec![34, 50, 25, 100, 65];
    let result = largest(&numbers);

    log::info!("The largest number from {:?} is {}", numbers, result);

    let chars = vec!['y', 'm', 'a', 'q'];
    let result = largest(&chars);
    log::info!("The largest chars from {:?} is {}", chars, result);

    log::info!("In Struct Definitions");
    let integer = Point { x: 5, y: 10 };
    let float: Point<f32, f32> = Point { x: 1.0, y: 4.0 };
    let mixed = Point { x: 5, y: 4.0 };
    log::debug!(
        "An integer Point {:?} with x being {} and y being {}",
        integer,
        integer.x,
        integer.y
    );
    log::debug!(
        "A float Point {:?} with x being {} and y being {}",
        float,
        float.x,
        float.y
    );
    log::debug!(
        "A mixed Point {:?} with x being {} and y being {}",
        mixed,
        mixed.x,
        mixed.y
    );

    log::info!("In Enum Definitions");
    log::info!("See Option<T> chapter 6 and Result<T, E> chapter 9");

    log::info!("In Method Definitions");
    log::info!(
        "Using generic methods for Point<T, U>, x is {} and y is {}",
        float.x(),
        float.y()
    );
    log::info!(
        "Using generic methods for Point<i32, f32>, x is {} and y is {}",
        mixed.x(),
        mixed.y()
    );
    log::info!(
        "Using generic methods for Point<i32, f32>, x is {} and y is {}",
        integer.x(),
        integer.y()
    );
    log::info!(
        "Using generic Method implemented only for Point<f32, f32>, distance from origin for {:?} is {}",
        float,
        float.distance_from_origin()
    );

    log::info!("Mixing Generic Types on Methods and Strucs definition");

    let p1 = MixPoint { x: 5, y: 10.4 };
    let p2 = MixPoint { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    log::info!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    log::info!("Traits: Defining shared Behavior");
    log::info!("Defining a Trait");
    log::info!("Implementing a Trait on a Type");

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("Of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    log::info!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
        ),
    };

    log::info!("New article available! {}", article.summarize());

    log::info!("Trait as Parameters type");
    notify(&tweet);
    notify(&article);
    mixed_feed(&tweet, &article);

    log::info!("Trait bound syntax");

    // feed(&tweet, &article); // Note: won't work as with Trait bound syntax both parameter need to be of the same type and not only implement the Trait
    // Note: with Trait bound both parameter need to be of the same type, not only implement the Trait specified in the Bound
    feed(
        &tweet,
        &Tweet {
            content: String::from("This is the last Tweet I will write"),
            username: String::from("John Doe"),
            reply: false,
            retweet: false,
        },
    );

    log::info!("Returning Types That Implement Traits");
    notify(&returns_summarizable());

    log::info!("Using Trait Bounds to Conditionally Implement Methods");

    let pair = Pair::new(3, 4);
    log::info!("pair x is {} and y is {}", pair.x(), pair.y());
    pair.cmp_display();
    let other_pair = Pair::new("foo", "bar");
    other_pair.cmp_display();
    let _wrong_pair = Pair::new(vec![1, 2, 3], vec![4, 5, 6]);
    // wrong_pair.cmp_display(); // Note: work as the Trait bound is not respected

    log::info!("Validating References with Lifetimes");

    log::info!("Preventing Dangling References with Lifetimes");
    // Note: dangling reference, won't compile
    //     let r;
    //     {
    //         let x = 5;
    //         r = &x;
    //     }
    //     log::info!("r: {}", r);
    // Note: no dangling refenrence -> compiles
    let x = 5;
    let r = &x;
    log::info!("r: {}", r);

    log::info!("Generic Lifetimes in Functions");
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    log::info!("The longest string is {result}");

    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
        log::info!("The longest string is {result}");
    }
    // log::info!("The longest string is {result}"); // Note: won't work as the lifetime or resultt is constrained by the lifetime of string2

    log::info!("Lifetime Annotations in Struct Definitions");
    let novel = "Call me Ishmael. Some years ago...".to_string();
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    log::info!("Important excerpt: {}", i.part);

    log::info!("Lifetime Elision");
    let exs = vec![
        "One",
        "One word",
        "Word for one",
        "One for all",
        "All for one",
    ];
    for ex in exs {
        log::info!("The first word of '{}' is '{}'", ex, first_word(&ex));
    }
}

#[cfg(test)]
mod tests {
    use super::longest;

    #[test]
    fn x_is_longer() {
        assert_eq!(longest("longer", "long"), "longer");
    }

    #[test]
    fn y_is_longer() {
        assert_eq!(longest("long", "longer"), "longer");
    }
}
