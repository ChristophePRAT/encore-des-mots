mod csv_helper;
mod letter_helper;

use std::collections::HashMap;

fn create_words(letters: Vec<char>, length: usize) -> Vec<String> {
    let mut results = Vec::new();

    // Base case: If length is 0, we return an empty string inside the list
    if length == 0 {
        return vec!["".to_string()];
    }

    // Recursive step
    let sub_words = create_words(letters.clone(), length - 1);

    for word in sub_words {
        for &c in &letters {
            let mut new_word = word.clone();
            new_word.push(c);
            results.push(new_word);
        }
    }

    results
}

fn count_letters(word: &str) -> HashMap<char, usize> {
    let mut counts = HashMap::new();
    for c in word.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }
    counts
}

fn all_words(words: Vec<String>) -> Vec<String> {
    let max_len = words.iter().map(|f| f.chars().count()).max().unwrap_or(0);
    let unique_letters = words
        .iter()
        .flat_map(|f| f.chars())
        .collect::<std::collections::HashSet<char>>()
        .into_iter()
        .collect::<Vec<char>>();

    // Compute max frequency of each letter across all words
    let mut max_letter_freq: HashMap<char, usize> = HashMap::new();
    for word in &words {
        let counts = count_letters(word);
        for (c, count) in counts {
            max_letter_freq
                .entry(c)
                .and_modify(|e| *e = (*e).max(count))
                .or_insert(count);
        }
    }

    println!(
        "words with length {} and within a set of {} chars",
        max_len,
        unique_letters.len()
    );

    // Generate all words for lengths 1 to max_len
    let mut result = Vec::new();
    for len in 1..=max_len {
        let words = create_words(unique_letters.clone(), len);
        // Filter words that respect max letter frequencies
        for word in words {
            let word_counts = count_letters(&word);
            let mut valid = true;
            for (c, count) in word_counts {
                if let Some(&max_count) = max_letter_freq.get(&c) {
                    if count > max_count {
                        valid = false;
                        break;
                    }
                } else {
                    // Character not in any of the record's words
                    valid = false;
                    break;
                }
            }
            if valid {
                result.push(word);
            }
        }
    }

    result
}
fn test() {
    let word1 = "hello";
    let word2 = "hallo";
    let dist = letter_helper::distance(word1, word2);
    println!("Distance between '{}' and '{}' is {}", word1, word2, dist);
}

fn minimize_trans_cost(words: &Vec<String>) -> String {
    // all_words takes ownership of a Vec<String>, so clone the vector here
    let possible_words = all_words(words.clone());
    println!("Generated {} possible words", possible_words.len());

    let best_fit = possible_words
        .iter()
        .min_by(|w1, w2| {
            // Pass references to total_distance to avoid moving `words`
            let dist1 = letter_helper::total_distance(w1.as_str(), words);
            let dist2 = letter_helper::total_distance(w2.as_str(), words);
            dist1.partial_cmp(&dist2).unwrap()
        })
        .unwrap();
    best_fit.clone()
}

fn get_fields(record: &csv_helper::Record) -> Vec<String> {
    vec![
        // record.bs.clone(),
        record.de.clone(),
        record.en.clone(),
        record.es.clone(),
        record.fr.clone(),
        record.it.clone(),
        // record.la.clone(),
        record.nl.clone(),
        record.pl.clone(),
        record.pt.clone(),
        // record.sk.clone(),
        // record.sl.clone(),
    ]
}
fn main() {
    let records = csv_helper::read_csv("swadesh/norm.csv").expect("Failed to read CSV file");
    println!(
        "CSV file read successfully, number of records: {}",
        records.len()
    );
    test();
    let fields = get_fields(&records[0]);
    let all_words = all_words(fields);

    // println!("All words: {:?}", all_words);

    println!("Minimizing...");
    let f = &get_fields(&records[11]);
    let best_word = minimize_trans_cost(f);
    println!("Best word: {} = {:?}", best_word, f);
}
