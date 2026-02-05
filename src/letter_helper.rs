use std::collections::HashMap;

pub fn distance(word1: &str, word2: &str) -> usize {
    let mut count1 = HashMap::new();
    for c in word1.chars() {
        *count1.entry(c).or_insert(0) += 1;
    }
    let mut count2 = HashMap::new();
    for c in word2.chars() {
        *count2.entry(c).or_insert(0) += 1;
    }
    let mut removals = 0;
    let mut additions = 0;
    for (&c, &cnt1) in &count1 {
        let cnt2 = count2.get(&c).unwrap_or(&0);
        if cnt1 > *cnt2 {
            removals += cnt1 - *cnt2;
        }
    }
    for (&c, &cnt2) in &count2 {
        let cnt1 = count1.get(&c).unwrap_or(&0);
        if cnt2 > *cnt1 {
            additions += cnt2 - *cnt1;
        }
    }
    removals + additions
}

pub fn complexity(word: &str) -> f64 {
    (word.chars().count() as f64) * (26_f64).log2()
}

pub fn transition_complexity(word1: String, word2: String) -> f64 {
    // K(w1, w2) = K(w2) + K(w1|w2)
    complexity(&word1) + (distance(&word1, &word2) as f64)
}
