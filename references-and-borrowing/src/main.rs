fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    let _s = String::from("hello");
    // change(&s);

    // Mutable references
    let mut s_mut = String::from("hello");
    change_mut(&mut s_mut);

    {
        let _r1 = &mut s_mut;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let _r2 = &mut s_mut; // without the scope for _r1 that would fail as we can only have one mutable reference at a time.

    let mut _s_mut_prime = String::from("Again");
    let _r1_prime = &_s_mut_prime;
    let _r2_prime = &_s_mut_prime;
    // let _r3_prime = &mut _s_mut_prime; // not allowed a _s_mut_prime is already borrowed as immutable
    // but if we release the immutable references using them
    println!("{} and {}", _r1_prime, _r2_prime);
    // variables _r1_prime and _r2_prime will not be used after this point

    let _r3_prime = &mut _s_mut_prime; // then it works!
    println!("{}", _r3_prime);
}

//  refer to some value without taking ownership of it
// s is a reference to a String
fn calculate_length(s: &String) -> usize {
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what it refers to, it is not dropped.

// Does not work as s is a & reference and is not mutable
// fn change(s: &String) {
//     s.push_str(", world!");
// }

fn change_mut(s: &mut String) {
    s.push_str(", world!");
}
