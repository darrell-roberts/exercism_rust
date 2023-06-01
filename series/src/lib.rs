pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len == 0 {
        return (0..=digits.len()).map(|_| "".into()).collect();
    }

    (1..digits.len())
        .enumerate()
        .map(|(skip, _)| digits.chars().skip(skip).take(len).collect::<String>())
        .take_while(|serie| serie.len() == len && !serie.is_empty())
        .collect()
}
