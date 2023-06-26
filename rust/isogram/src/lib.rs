use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let mut letters = HashSet::new();

    for c in candidate
        .chars()
        .filter(|c| c.is_alphabetic())
        .flat_map(|c| c.to_lowercase())
    {
        if letters.contains(&c) {
            return false;
        }
        letters.insert(c);
    }

    true
}
