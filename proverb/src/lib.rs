pub fn build_proverb(list: Vec<&str>) -> String {
    match list.get(0) {
        Some(word) => list
            .windows(2)
            .map(|words| format!("For want of a {} the {} was lost.\n", words[0], words[1]))
            .chain(std::iter::once(format!(
                "And all for the want of a {}.",
                word
            )))
            .collect::<String>(),
        None => String::new(),
    }
}
