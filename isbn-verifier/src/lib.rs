/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    isbn.chars()
        .filter(|&c| c != '-')
        .fuse()
        .enumerate()
        .map(|(i, c)| match (c, i) {
            (_, i) if i > 9 => None,
            ('X', 9) => Some(10),
            (c, _) => c.to_digit(10).map(|d| d as usize * (10 - i)),
        })
        .try_fold(0usize, |acc, c| c.map(|c| acc + c))
        .map_or(false, |sum| sum % 11 == 0)
}
