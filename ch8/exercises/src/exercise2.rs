//Convert strings to pig latin. The first consonant of each word is moved
// to the end of the word and ay is added, so first becomes irst-fay.
//  Words that start with a vowel have hay added to the end instead
// (apple becomes apple-hay). Keep in mind the details about UTF-8 encoding!

use std::io;
fn is_vowel(c: char) -> bool {
    matches!(c, 'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U')
}
pub fn exercise_2() -> String {
    //let the user input a string
    println!("Enter a string: ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim();

    //split the string into words
    let words = input.split_whitespace();
    let mut pig_latin_words = Vec::new();
    //convert each word to pig latin
    for word in words {
        let mut chars = word.chars();

        if let Some(first_char) = chars.next() {
            let rest: String = chars.collect();
            let pig_word = if is_vowel(first_char) {
                format!("{}-hay", word.to_lowercase())
            } else {
                format!("{}-{}ay", rest.to_lowercase(), first_char.to_lowercase())
            };
            pig_latin_words.push(pig_word);
        }
    }
    pig_latin_words.join(" ")
}
