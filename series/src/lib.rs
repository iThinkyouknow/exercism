pub fn series(digits: &str, len: usize) -> Vec<String> {
    let digits_len = digits.len();
    match digits_len >= len {
        true => (0..=(digits_len - len))
            .map(|index| String::from(&digits[index..(index + len)]))
            .collect::<Vec<String>>(),
        false => Vec::new(),
    }
}
