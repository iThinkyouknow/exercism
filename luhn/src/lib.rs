use std::char;
/// Check a Luhn checksum.
// adapted from @JaneL's
pub fn is_valid(code: &str) -> bool {
    let code = code.replace(char::is_whitespace, "");

    match code.len() < 2 {
        true => false,
        _ => code
            .chars()
            .rev()
            .map(|c| c.to_digit(10))
            .enumerate()
            .try_fold(0, |acc, (index, maybe_d)| {
                maybe_d
                    .map(|d| match index % 2 == 1 {
                        true => d * 2,
                        false => d,
                    })
                    .map(|d| match d > 9 {
                        true => d - 9,
                        false => d,
                    })
                    .map(|d| acc + d)
            })
            .map_or(false, |sum| sum % 10 == 0),
    }
}
