/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    ('a'..='z').all(|c| {
        sentence
            .chars()
            .flat_map(|c| c.to_lowercase())
            .any(|l| l == c)
    })
}
