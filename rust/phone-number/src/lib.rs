pub fn number(user_number: &str) -> Option<String> {
    let num_string = user_number
        .chars()
        .filter(|c| c.is_numeric())
        .collect::<String>();

    let length = (num_string.len() == 10 || num_string.len() == 11).then_some(num_string.len())?;

    num_string
        .chars()
        .rev()
        .zip(1..)
        .all(|(c, i)| {
            // First digit area code or first digit exchange code.
            if i == 7 || i == 10 {
                (2..9).any(|n| c.to_digit(10).map(|c| c == n).unwrap_or(false))
            } else {
                true
            }
        })
        .then_some(())?;

    match length {
        10 => Some(num_string),
        11 if num_string.chars().next().map(|c| c == '1').unwrap_or(false) => {
            Some(String::from(&num_string[1..]))
        }
        _ => None,
    }
}
