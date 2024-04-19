fn main() {
    println!("# Twelve Days of Christmas");

    // Song pattern
    // On the first day of Christmas my true love sent to me
    // A partridge in a pear tree

    // On the second day of Christmas my true love sent to me
    // Two turtle doves,
    // And a partridge in a pear tree.

    // On the third day of Christmas my true love sent to me
    // Three French hens,
    // Two turtle doves,
    // And a partridge in a pear tree.

    // Items
    // a partridge in a pear tree
    // two turtle doves,
    // three French hens,
    // four calling birds
    // five gold rings
    // six geese a-laying
    // seven swans a-swimming
    // eight maids a-milking
    // nine ladies dancing
    // ten lords a-leaping
    // eleven pipers piping
    // twelve drummers drumming

    let ns = [
        ["one", "first"],
        ["two", "second"],
        ["three", "third"],
        ["four", "fourth"],
        ["five", "fifth"],
        ["six", "sixth"],
        ["seven", "seventh"],
        ["eight", "eighth"],
        ["nine", "ninth"],
        ["ten", "tenth"],
        ["eleven", "eleventh"],
        ["twelve", "twelfth"],
    ];

    for c in ns {
        println!("{}: {}", c[0], c[1]);
    }
}
