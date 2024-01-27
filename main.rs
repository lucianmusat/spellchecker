mod spellchecker;

use spellchecker::Spellchecker;

fn main() {
    let sentence = "My name is proffessor Lucian";
    let mut spellchecked_sentence = String::new();
    let spellchecker = Spellchecker::new("dictionary.txt");
    match spellchecker {
        Some(spellchecker) => {
            for word in sentence.split_whitespace() {
                let result = spellchecker.spellcheck(&word.to_lowercase());
                let mut spellchecked_word = match result {
                    Some(word) => word,
                    None => word.to_string(),
                };
                capitalize_if_needed(&word, &mut spellchecked_word);
                spellchecked_sentence.push_str(&spellchecked_word);
                spellchecked_sentence.push_str(" ");
            }
        }
        None => println!("Could not create spellchecker!"),
    }
    println!("Spellchecked: {}", spellchecked_sentence);
}

fn capitalize_if_needed(original_word: &str, spellchecked_word: &mut String) {
    if original_word.chars().next().unwrap().is_uppercase() {
        let first_char = spellchecked_word.chars().next().unwrap().to_uppercase().to_string();
        spellchecked_word.replace_range(..1, &first_char);
    }
}