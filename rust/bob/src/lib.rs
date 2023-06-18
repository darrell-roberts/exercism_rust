pub fn reply(message: &str) -> &str {
    let trimmed = message.trim();
    let question = trimmed.chars().last().map(|c| c == '?').unwrap_or(false);
    let yelling = trimmed.chars().any(|c| c.is_alphabetic())
        && trimmed
            .chars()
            .filter(|c| c.is_alphabetic())
            .all(|c| c.is_uppercase());

    match (question, yelling) {
        (true, true) => "Calm down, I know what I'm doing!",
        (true, false) => "Sure.",
        (false, true) => "Whoa, chill out!",
        (false, false) if trimmed.is_empty() => "Fine. Be that way!",
        _ => "Whatever.",
    }
}
