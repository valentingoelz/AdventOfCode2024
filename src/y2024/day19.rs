use std::collections::{HashMap, HashSet};

fn is_valid(pattern: &str, words: &HashSet<String>, cache: &mut HashMap<String, i64>) -> i64 {
    // Check if the pattern is cached
    if let Some(&count) = cache.get(pattern) {
        return count;
    }

    // Base case: empty pattern
    if pattern.is_empty() {
        return 1;
    }

    let mut count: i64 = 0;

    // Iterate over all possible splits of the pattern
    for i in 1..=pattern.len() {
        let prefix = &pattern[..i];
        let suffix = &pattern[i..];

        // Check if the prefix is in the words set
        if words.contains(prefix) {
            count += is_valid(suffix, words, cache);
        }
    }

    // Cache the result for the current pattern
    cache.insert(pattern.to_string(), count);
    count
}

pub fn solve(input: String) {
    // Split the input into lines
    let mut lines = input.lines();

    // Read the first line and populate the words set
    let words_line = lines.next().unwrap();
    let words: HashSet<String> = words_line.split(", ").map(String::from).collect();

    // Skip the second line
    lines.next();

    // Read the remaining lines as patterns
    let patterns: Vec<String> = lines.map(|line| line.trim().to_string()).collect();

    // Initialize cache and count
    let mut cache = HashMap::new();
    let mut count = 0;

    // Process each pattern
    for pattern in patterns {
        count += is_valid(&pattern, &words, &mut cache);
    }

    // Print the result
    println!("{}", count);
}
