pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = Vec::new();
    !string.chars().any(|c| match c {
        ']' | ')' | '}' => stack.pop() != Some(c),
        '[' => {
            stack.push(']');
            false
        }
        '(' => {
            stack.push(')');
            false
        }
        '{' => {
            stack.push('}');
            false
        }
        _ => false,
    }) && stack.is_empty()
}
