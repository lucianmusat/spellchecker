use crate::spellchecker::Spellchecker;
use crate::spellchecked::Spellchecked;

use log::debug;
use rayon::prelude::*;

// The SpellcheckParser is responsible for parsing the input text and spellchecking it
// It uses the Spellchecker to spellcheck individual words
pub struct SpellcheckParser {
    spellchecker: Spellchecker,
}

impl SpellcheckParser {

    pub fn new() -> SpellcheckParser {
        let spellchecker = Spellchecker::new("dictionary.txt");
        match spellchecker {
            Some(spellchecker) => {
                SpellcheckParser { spellchecker }
            }
            None => panic!("Could not create spellchecker"),
        }
    }

    pub fn spellcheck_all(&self, to_spellcheck: &str) -> Vec<Spellchecked> {
        to_spellcheck
            .split_whitespace()
            .collect::<Vec<&str>>()
            // Secret sauce is here, Rayon creates a parallel iterator
            .par_iter()
            .map(|word| self.process_word(word))
            .collect()
    }

    fn process_word(&self, original_word: &str) -> Spellchecked {
        debug!("Processing word: {}", original_word);
        match original_word.chars().all(|c| c.is_alphabetic()) {
            true => {
                self.spellcheck_word(original_word).unwrap()
            }
            false => Spellchecked {
                original: original_word.to_string(),
                spellchecked: self.spellcheck_with_punctuation(original_word),
            },
        }
    }

    fn spellcheck_word(&self, original_word: &str) -> Option<Spellchecked> {
        if !original_word.is_empty() { () }
        let result = self.spellchecker.spellcheck(&original_word.to_lowercase());
        let mut spellchecked_word = match result {
            Some(word) => word,
            None => original_word.to_string(),
        };
        self.capitalize_if_needed(&original_word, &mut spellchecked_word);
        Spellchecked {
            original: original_word.to_string(),
            spellchecked: spellchecked_word,
        }.into()
    }

    fn capitalize_if_needed(&self, original_word: &str, spellchecked_word: &mut String) {
        if original_word.is_empty() {
            return;
        }
        if original_word.chars().next().unwrap().is_uppercase() {
            let first_char = spellchecked_word.chars().next().unwrap().to_uppercase().to_string();
            spellchecked_word.replace_range(..1, &first_char);
        }
    }

    fn split_by_alphabetic(text: &str) -> Vec<&str> {
        if text.is_empty() {
            return Vec::new();
        }
        let mut split = Vec::new();
        let mut word_start = 0;

        for (i, char) in text.chars().enumerate() {
            if char.is_alphabetic() != (i > 0 && text.chars().nth(i - 1).unwrap().is_alphabetic()) {
                split.push(&text[word_start..i]);
                word_start = i;
            }
        }

        split.push(&text[word_start..]);
        split.retain(|&word| !word.is_empty());
        split
    }

    fn spellcheck_with_punctuation(&self,  original_word: &str) -> String {
        let mut spellchecked_word_with_punctuation = Vec::new();
        for word in Self::split_by_alphabetic(original_word) {
            if word.chars().all(|c| c.is_alphabetic()) {
                let spellchecked = self.spellcheck_word(&word);
                if let Some(spellchecked) = spellchecked {
                    spellchecked_word_with_punctuation.push(spellchecked);
                }
            } else {
                spellchecked_word_with_punctuation.push(Spellchecked {
                    original: word.to_string(),
                    spellchecked: word.to_string(),
                });
            }
        }
        let spellchecked_word = spellchecked_word_with_punctuation
            .iter()
            .map(|word| word.spellchecked.clone())
            .collect::<String>();
        spellchecked_word
    }

}
