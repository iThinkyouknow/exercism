use std::char;
use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    words
        .to_lowercase()
        .split(|c: char| !c.is_alphanumeric())
        .filter(|word| !word.is_empty())
        .fold(HashMap::new(), |mut h_map, word| {
            h_map
                .entry(String::from(word))
                .and_modify(|e| *e += 1)
                .or_insert(1);
            h_map
        })
}
