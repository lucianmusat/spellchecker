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

    pub fn new() -> Result<SpellcheckParser, String> {
        let spellchecker = Spellchecker::new("dictionary.txt");
        match spellchecker {
            Ok(spellchecker) => Ok(SpellcheckParser {
                spellchecker,
            }),
            Err(err_message) => Err(err_message),
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
        if original_word.is_empty() { return None; }
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_process_word() {
        let spellcheck_parser = SpellcheckParser::new().unwrap();
        let word = "hello";
        let result = spellcheck_parser.process_word(word);
        assert_eq!(result.original, "hello");
        assert_eq!(result.spellchecked, "hello");
    }

    #[test]
    fn test_process_word_with_punctuation() {
        let spellcheck_parser = SpellcheckParser::new().unwrap();
        let word = "hello,";
        let result = spellcheck_parser.process_word(word);
        assert_eq!(result.original, "hello,");
        assert_eq!(result.spellchecked, "hello,");
    }

    #[test]
    fn test_process_word_with_punctuation_and_space() {
        let spellcheck_parser = SpellcheckParser::new().unwrap();
        let word = "hello, ";
        let result = spellcheck_parser.process_word(word);
        assert_eq!(result.original, "hello, ");
        assert_eq!(result.spellchecked, "hello, ");
    }

    #[test]
    fn test_process_word_with_punctuation_and_space_and_word() {
        let spellcheck_parser = SpellcheckParser::new().unwrap();
        let word = "hello, wrld";
        let result = spellcheck_parser.process_word(word);
        assert_eq!(result.original, "hello, wrld");
        assert_eq!(result.spellchecked, "hello, world");
    }

    #[test]
    fn test_process_word_with_punctuation_and_space_and_word_and_punctuation() {
        let spellcheck_parser = SpellcheckParser::new().unwrap();
        let word = "hello, wrld!";
        let result = spellcheck_parser.process_word(word);
        assert_eq!(result.original, "hello, wrld!");
        assert_eq!(result.spellchecked, "hello, world!");
    }

    #[test]
    fn test_spellcheck_all_empty() {
        let spellcheck_parser = SpellcheckParser::new().unwrap();
        let to_spellcheck = "";
        let result = spellcheck_parser.spellcheck_all(to_spellcheck);
        assert!(result.is_empty());
    }

    #[test]
    fn test_spellcheck_all() {
        let spellcheck_parser = SpellcheckParser::new().unwrap();
        let to_spellcheck = "hello, wrld!";
        let result = spellcheck_parser.spellcheck_all(to_spellcheck);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].original, "hello,");
        assert_eq!(result[0].spellchecked, "hello,");
        assert_eq!(result[1].original, "wrld!");
        assert_eq!(result[1].spellchecked, "world!");
    }

    #[test]
    fn spellcheck_word_empty() {
        let spellcheck_parser = SpellcheckParser::new().unwrap();
        let word = "";
        let result = spellcheck_parser.spellcheck_word(word);
        assert_eq!(result, None);
    }

    #[test]
    fn spellcheck_word() {
        let spellcheck_parser = SpellcheckParser::new().unwrap();
        let word = "speling";
        let result = spellcheck_parser.spellcheck_word(word);
        assert_eq!(result, Some(Spellchecked {
            original: "speling".to_string(),
            spellchecked: "spelling".to_string(),
        }));
    }

    #[test]
    fn spellcheck_word_capitalized() {
        let spellcheck_parser = SpellcheckParser::new().unwrap();
        let word = "Speling";
        let result = spellcheck_parser.spellcheck_word(word);
        assert_eq!(result, Some(Spellchecked {
            original: "Speling".to_string(),
            spellchecked: "Spelling".to_string(),
        }));
    }

    #[test]
    fn spellcheck_word_no_close_match() {
        let spellcheck_parser = SpellcheckParser::new().unwrap();
        let word = "zzz";
        let result = spellcheck_parser.spellcheck_word(word);
        assert_eq!(result, Some(Spellchecked {
            original: "zzz".to_string(),
            spellchecked: "z".to_string(),
        }));
    }

    #[test]
    fn spellcheck_with_punctuation() {
        let spellcheck_parser = SpellcheckParser::new().unwrap();
        let word = "hello, wrld!";
        let result = spellcheck_parser.spellcheck_with_punctuation(word);
        assert_eq!(result, "hello, world!");
    }

    #[test]
    fn spellcheck_with_punctuation_no_punctuation() {
        let spellcheck_parser = SpellcheckParser::new().unwrap();
        let word = "hello world";
        let result = spellcheck_parser.spellcheck_with_punctuation(word);
        assert_eq!(result, "hello world");
    }

    #[test]
    fn split_by_alphabetic_empty() {
        let result = SpellcheckParser::split_by_alphabetic("");
        assert!(result.is_empty());
    }

    #[test]
    fn split_by_alphabetic() {
        let result = SpellcheckParser::split_by_alphabetic("hello, world!");
        assert_eq!(result, vec!["hello", ", ", "world", "!"]);
    }

    #[test]
    fn split_by_alphabetic_no_punctuation() {
        let result = SpellcheckParser::split_by_alphabetic("hello world");
        assert_eq!(result, vec!["hello", " ", "world"]);
    }

    #[test]
    fn split_by_alphabetic_one_word() {
        let result = SpellcheckParser::split_by_alphabetic("Incomprehensibility");
        assert_eq!(result, vec!["Incomprehensibility"]);
    }

    #[test]
    fn split_by_alphabetic_one_word_with_punctuation() {
        let result = SpellcheckParser::split_by_alphabetic("Incomprehensibility!");
        assert_eq!(result, vec!["Incomprehensibility", "!"]);
    }

    #[test]
    fn capitalize_if_needed() {
        let spellcheck_parser = SpellcheckParser::new().unwrap();
        let original_word = "Speling";
        let mut spellchecked_word = "spelling".to_string();
        spellcheck_parser.capitalize_if_needed(original_word, &mut spellchecked_word);
        assert_eq!(spellchecked_word, "Spelling");
    }

    #[test]
    fn capitalize_if_needed_not_needed() {
        let spellcheck_parser = SpellcheckParser::new().unwrap();
        let original_word = "";
        let mut spellchecked_word = "spelling".to_string();
        spellcheck_parser.capitalize_if_needed(original_word, &mut spellchecked_word);
        assert_eq!(spellchecked_word, "spelling");
    }

    #[test]
    fn capitalize_if_needed_empty() {
        let spellcheck_parser = SpellcheckParser::new().unwrap();
        let original_word = "";
        let mut spellchecked_word = "".to_string();
        spellcheck_parser.capitalize_if_needed(original_word, &mut spellchecked_word);
        assert_eq!(spellchecked_word, "");
    }
}
