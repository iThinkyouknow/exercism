#[derive(Debug, PartialEq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    match (span == 0, string_digits.len() < span) {
        (true, _) => Ok(1),
        (_, true) => Err(Error::SpanTooLong),
        (_, _) => Ok(string_digits
            .chars()
            .map(|c| {
                c.to_digit(10)
                    .map(|d| d as u64)
                    .ok_or(Error::InvalidDigit(c))
            })
            .collect::<Result<Vec<u64>, Error>>()?
            .windows(span)
            .map(|w| w.iter().product::<u64>())
            .max()
            .unwrap()),
    }
}
