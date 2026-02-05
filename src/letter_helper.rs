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

pub fn levenshtein_distance(word1: &str, word2: &str) -> usize {
    let len1 = word1.chars().count();
    let len2 = word2.chars().count();

    // Create a matrix: (len1 + 1) x (len2 + 1)
    // dp[i][j] is the distance between word1[0..i] and word2[0..j]
    let mut dp = vec![vec![0; len2 + 1]; len1 + 1];

    // Initialize base cases:
    // Transforming a word to an empty string requires i removals
    for i in 0..=len1 {
        dp[i][0] = i;
    }
    // Transforming an empty string to a word requires j additions
    for j in 0..=len2 {
        dp[0][j] = j;
    }

    let chars1: Vec<char> = word1.chars().collect();
    let chars2: Vec<char> = word2.chars().collect();

    for i in 1..=len1 {
        for j in 1..=len2 {
            if chars1[i - 1] == chars2[j - 1] {
                // Characters match, no new operation needed
                dp[i][j] = dp[i - 1][j - 1];
            } else {
                // Characters differ: choose the minimum of:
                // 1. Remove from word1 (dp[i-1][j] + 1)
                // 2. Add to word1 (dp[i][j-1] + 1)
                // Note: Standard Levenshtein also includes 'Substitution'.
                // If you ONLY want add/remove, we ignore the diagonal + 1.
                dp[i][j] = 1 + std::cmp::min(dp[i - 1][j], dp[i][j - 1]);
            }
        }
    }

    dp[len1][len2]
}

pub fn levenshtein_distance_sub(word1: &str, word2: &str) -> usize {
    let len1 = word1.chars().count();
    let len2 = word2.chars().count();

    // Handle edge cases where one string is empty
    if len1 == 0 {
        return len2;
    }
    if len2 == 0 {
        return len1;
    }

    // prev_row represents the previous row of the DP matrix
    let mut prev_row: Vec<usize> = (0..=len2).collect();
    let mut curr_row = vec![0; len2 + 1];

    let chars1: Vec<char> = word1.chars().collect();
    let chars2: Vec<char> = word2.chars().collect();

    for i in 1..=len1 {
        // First element of current row is the distance from empty string
        curr_row[0] = i;

        for j in 1..=len2 {
            if chars1[i - 1] == chars2[j - 1] {
                // Characters are the same; no cost added
                curr_row[j] = prev_row[j - 1];
            } else {
                // Characters differ; pick the minimum of the three operations
                let substitution = prev_row[j - 1] + 1;
                let insertion = curr_row[j - 1] + 1;
                let deletion = prev_row[j] + 1;

                curr_row[j] = substitution.min(insertion).min(deletion);
            }
        }
        // Move current row to previous for the next iteration
        prev_row.copy_from_slice(&curr_row);
    }

    prev_row[len2]
}

pub fn complexity(word: &str) -> f64 {
    (word.chars().count() as f64) * (26_f64).log2()
}

pub fn transition_complexity(word1: &str, word2: &str) -> f64 {
    // K(w1, w2) = K(w2) + K(w1|w2)
    // (distance(word1, word2) as f64)
    levenshtein_distance_sub(word1, word2) as f64
}

pub fn total_distance(word: &str, words: &Vec<String>) -> f64 {
    words
        .iter()
        .map(|w| transition_complexity(word, w.as_str()))
        .sum()
}
