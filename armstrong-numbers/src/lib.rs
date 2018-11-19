pub fn is_armstrong_number(num: u32) -> bool {
    let num_string = num.to_string();
    let num_len = num_string.len() as u32;

    let digits_sum = num_string.chars().fold(0, |acc, digit_char| {
        acc + (digit_char.to_digit(10).unwrap()).pow(num_len)
    });
    digits_sum == num
}
