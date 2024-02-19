use std::fs;
use std::io::{BufRead, BufReader};

const MIN_DISTANCE: usize = 1000;

pub struct Spellchecker {
    dictionary : Vec<String>
}

impl Spellchecker {

    pub fn new(dictionary_file: &str) -> Result<Spellchecker, String> {
        if !fs::metadata(dictionary_file).is_ok() {
            return Err(format!("Dictionary file '{}' not found", dictionary_file));
        }
        let mut dictionary = Vec::new();
        let reader = BufReader::new(fs::File::open(&dictionary_file)
                                    .map_err(|e| format!("Could not open file: {}", e))?);
        for line in reader.lines() {
            let line = line.expect("Could not read line");
            dictionary.push(line);
        }
        Ok(Spellchecker { dictionary })
    }

    pub fn spellcheck(&self, word: &str) -> Option<String> {
        if word.is_empty() { return None; }
        let mut min_distance = MIN_DISTANCE;
        let mut closest_word = None;

        for dict_word in &self.dictionary {
            let distance = self.wagner_fischer(word, &dict_word);
            if distance < min_distance {
                min_distance = distance;
                closest_word = Some(dict_word.to_string());
            }
        }
        closest_word
    }

    // Use the Wagner-Fischer algorithm to calculate the edit distance between two strings
    fn wagner_fischer(&self, a: &str, b: &str) -> usize {
        let mut matrix = vec![vec![0; b.len() + 1]; a.len() + 1];

        for i in 0..a.len() + 1 {
            matrix[i][0] = i;
        }

        for j in 0..b.len() + 1 {
            matrix[0][j] = j;
        }

        for i in 1..a.len() + 1 {
            for j in 1..b.len() + 1 {
                let indicator = if a.chars().nth(i - 1) == b.chars().nth(j - 1) {
                    0
                } else {
                    1
                };

                matrix[i][j] = *[
                    matrix[i - 1][j] + 1,
                    matrix[i][j - 1] + 1,
                    matrix[i - 1][j - 1] + indicator,
                ]
                .iter()
                .min()
                .unwrap();
            }
        }

        matrix[a.len()][b.len()]
    }

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_spellchecker_new() {
        let spellchecker = Spellchecker::new("dictionary.txt");
        assert!(spellchecker.is_ok());
    }

    #[test]
    fn test_spellchecker_new_invalid_file() {
        let invalid_file = "nonexistent_file.txt";
        let spellchecker = Spellchecker::new(invalid_file);
        assert!(spellchecker.is_err());
        assert_eq!(
            spellchecker.err().unwrap(),
            format!("Dictionary file '{}' not found", invalid_file)
        );
    }

    #[test]
    fn test_spellchecker_spellcheck() {
        let spellchecker = Spellchecker::new("dictionary.txt").unwrap();
        let result = spellchecker.spellcheck("speling");
        assert_eq!(result, Some("spelling".to_string()));
    }

    #[test]
    fn test_spellchecker_spellcheck_empty_word() {
        let spellchecker = Spellchecker::new("dictionary.txt").unwrap();
        let result = spellchecker.spellcheck("");
        assert_eq!(result, None);
    }

    #[test]
    fn test_spellchecker_spellcheck_no_close_match() {
        let spellchecker = Spellchecker::new("dictionary.txt").unwrap();
        let result = spellchecker.spellcheck("zzz");
        assert_eq!(result, Some('z'.to_string()));
    }

    #[test]
    fn test_wagner_fischer() {
        let spellchecker = Spellchecker::new("dictionary.txt").unwrap();
        let result = spellchecker.wagner_fischer("kitten", "sitting");
        assert_eq!(result, 3);
    }

    #[test]
    fn test_wagner_fischer_empty_string() {
        let spellchecker = Spellchecker::new("dictionary.txt").unwrap();
        let result = spellchecker.wagner_fischer("", "sitting");
        assert_eq!(result, 7);
    }

    #[test]
    fn test_wagner_fischer_empty_string_2() {
        let spellchecker = Spellchecker::new("dictionary.txt").unwrap();
        let result = spellchecker.wagner_fischer("kitten", "");
        assert_eq!(result, 6);
    }

    #[test]
    fn test_wagner_fischer_empty_strings() {
        let spellchecker = Spellchecker::new("dictionary.txt").unwrap();
        let result = spellchecker.wagner_fischer("", "");
        assert_eq!(result, 0);
    }

    #[test]
    fn test_wagner_fischer_same_strings() {
        let spellchecker = Spellchecker::new("dictionary.txt").unwrap();
        let result = spellchecker.wagner_fischer("kitten", "kitten");
        assert_eq!(result, 0);
    }

}