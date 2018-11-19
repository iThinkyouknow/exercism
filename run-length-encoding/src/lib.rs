fn get_display_num(n: usize) -> String {
    match n {
        0 | 1 => String::new(),
        n => n.to_string(),
    }
}

pub fn encode(source: &str) -> String {
    let encoded = source.chars().enumerate().fold(
        (0usize, String::new(), String::new()),
        |(n, pending, result), (index, c)| {
            let s = String::new();
            match (index == source.len() - 1, pending == c.to_string()) {
                (false, false) => (
                    1,
                    c.to_string(),
                    format!("{}{}{}", result, get_display_num(n), pending),
                ),
                (false, true) => (n + 1, c.to_string(), result),
                (true, true) => (
                    0,
                    s,
                    format!("{}{}{}", result, get_display_num(n + 1), pending),
                ),
                (true, false) => (
                    0,
                    s,
                    format!(
                        "{}{}{}{}",
                        result,
                        get_display_num(n),
                        pending,
                        c.to_string()
                    ),
                ),
            }
        },
    );

    encoded.2
}

pub fn decode(source: &str) -> String {
    let decoded = source.chars().fold(
        (String::from(""), String::new()),
        |(mut times, result), c| match c.is_numeric() {
            true => {
                times.push(c);
                (times, result)
            }
            false => (
                String::from(""),
                format!(
                    "{}{}",
                    result,
                    c.to_string().repeat(times.parse::<usize>().unwrap_or(1))
                ),
            ),
        },
    );

    decoded.1
}
