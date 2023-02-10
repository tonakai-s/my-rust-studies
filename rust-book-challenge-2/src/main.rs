//Convert strings to pig latin.
//The first consonant of each word is moved to the end of the word
//and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel
//have “hay” added to the end instead (“apple” becomes “apple-hay”).
//Keep in mind the details about UTF-8 encoding!

fn main() {
    let my_string_to_convert = String::from("This Is My Simple Phrase To Convert Intris");
    let mut words: Vec<String> = my_string_to_convert.split_whitespace()
                                                     .map(str::to_string)
                                                    .collect();

    for word in &mut words {
        let ch = word.chars().next().unwrap();

        let is_vowel = "aeiouAEIOU".contains(ch);

        if is_vowel {
            word.push_str(&format!("-hay"));
        } else {
            *word = word[1..].to_string();
            word.push_str(&format!("-{ch}ay"))
        }
    }

    println!("{:?}", words);
}