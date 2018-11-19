fn is_yelling(message: &str) -> bool {
    message.to_uppercase() == message && message.chars().any(char::is_alphabetic)
}

fn is_question(message: &str) -> bool {
    message.ends_with("?")
}

pub fn reply(message: &str) -> &str {
    let m = message.trim();
    match (is_yelling(m), is_question(m), m.is_empty()) {
        (true, true, _) => "Calm down, I know what I'm doing!",
        (true, _, _) => "Whoa, chill out!",
        (_, true, _) => "Sure.",
        (_, _, true) => "Fine. Be that way!",
        _ => "Whatever.",
    }
}
