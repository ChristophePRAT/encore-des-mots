mod csv_helper;
mod letter_helper;

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

fn all_words(rec: &csv_helper::Record) -> Vec<String> {
    let fields = [
        &rec.sw, &rec.pl, &rec.en, &rec.fr, &rec.la, &rec.nl, &rec.bs, &rec.ca, &rec.es, &rec.sk,
        &rec.pt, &rec.hr, &rec.it, &rec.sl, &rec.de,
    ];

    let max_len = fields.iter().map(|f| f.chars().count()).max().unwrap_or(0);
    let unique_letters = fields
        .iter()
        .flat_map(|f| f.chars())
        .collect::<std::collections::HashSet<char>>()
        .into_iter()
        .collect::<Vec<char>>();
    println!(
        "words with length {} and within a set of {} chars",
        max_len,
        unique_letters.len()
    );

    return create_words(unique_letters, max_len);
}
fn test() {
    let word1 = "hello";
    let word2 = "hallo";
    let dist = letter_helper::distance(word1, word2);
    println!("Distance between '{}' and '{}' is {}", word1, word2, dist);
}

fn main() {
    let records = csv_helper::read_csv("swadesh/norm.csv").expect("Failed to read CSV file");
    println!(
        "CSV file read successfully, number of records: {}",
        records.len()
    );
    test();
    let all_words = all_words(&(records[0]));

    println!("All words: {:?}", all_words);
}
