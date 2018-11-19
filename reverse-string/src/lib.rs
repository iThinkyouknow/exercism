pub fn reverse(input: &str) -> String {
    // sorry, no grapheme handling here. Would need to install extern crate.
    input.chars().rev().collect()
}
