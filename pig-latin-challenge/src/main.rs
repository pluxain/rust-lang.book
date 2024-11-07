// Convert strings to pig latin.
// The first consonant of each word is moved to the end of the word and ay is added, so first becomes `irst-fay`.
// Words that start with a vowel have hay added to the end instead (apple becomes `apple-hay`).
// Keep in mind the details about UTF-8 encoding!

use is_vowel::{self, IsRomanceVowel};
use log;
use log4rs;
use std::{collections::HashSet, io::stdin};

fn t(input: &str) -> String {
    let word = input.trim();
    let mut chars = word.chars();
    let first_letter = chars.next().unwrap();
    let rest = chars.as_str();
    // Note: not included by default in the lib
    let extra_vowels: HashSet<char> = "yY".chars().collect();
    match first_letter {
        _c if first_letter.is_numeric() => word.to_string(),
        _c if first_letter.is_ascii_punctuation() => word.to_string(),
        c if first_letter.is_romance_vowel_including(&extra_vowels) => {
            format!("{}{}-hay", c, rest)
        }

        c => format!("{}-{}ay", rest, c),
    }
}

fn main() {
    log4rs::init_file("config/logger.yaml", Default::default()).unwrap();
    log::info!("Challenge: Pig Latin");

    loop {
        println!("Please enter a sentence:");
        let mut input = String::new();
        stdin()
            .read_line(&mut input) // read line from io into a mutable reference `&mut var`
            .expect("Failed to read your input from `stdin`");

        println!("Your sentence is `{}`", input.trim());
        // Note: create the vector from the input by splitting on whitespace
        let pig_latin = input
            .trim()
            .split_whitespace()
            .map(|w| t(w))
            .collect::<Vec<String>>()
            .join(" ");

        println!("Your translated sentence is `{}`", pig_latin);
        break;
    }
}

#[cfg(test)]
mod tests {
    use super::t;

    #[test]
    fn translate_word() {
        assert_eq!(t("first"), "irst-fay");
        assert_eq!(t("second"), "econd-say");
    }

    #[test]
    fn translate_word_starting_with_vowels() {
        assert_eq!(t("apple"), "apple-hay");
        assert_eq!(t("epple"), "epple-hay");
        assert_eq!(t("ipple"), "ipple-hay");
        assert_eq!(t("opple"), "opple-hay");
        assert_eq!(t("upple"), "upple-hay");
        assert_eq!(t("ypple"), "ypple-hay");
        assert_eq!(t("épple"), "épple-hay");
        assert_eq!(t("Ypple"), "Ypple-hay");
        assert_eq!(t("Àpple"), "Àpple-hay");
    }

    #[test]
    fn do_not_translate_numbers() {
        assert_eq!(t("234"), "234");
        assert_eq!(t("456.34"), "456.34");
    }

    #[test]
    fn do_not_translate_punctuation() {
        assert_eq!(t("!"), "!");
        assert_eq!(t("?!"), "?!");
        assert_eq!(t(" ! "), "!");
    }
}
