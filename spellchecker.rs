use std::fs;
use std::io::{BufRead, BufReader};


pub struct Spellchecker {
    dictionary : Vec<String>
}

impl Spellchecker {

    pub fn new(dictionary_file: &str) -> Spellchecker {
        if !fs::metadata(dictionary_file).is_ok() {
            panic!("Dictionary file not found");
        }
        let mut dictionary = Vec::new();
        let reader = BufReader::new(fs::File::open(&dictionary_file).unwrap());
        for line in reader.lines() {
            let line = line.expect("Could not read line");
            dictionary.push(line);
        }
        Spellchecker { dictionary }
    }

    pub fn spellcheck(&self, word: &str) -> Option<String> {
        let mut min_distance = 1000;
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