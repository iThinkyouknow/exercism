use std::collections::HashSet;
pub fn check(candidate: &str) -> bool {
    let mut h_set = HashSet::new();

    !candidate
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphabetic())
        .any(|c| !h_set.insert(c))
}
