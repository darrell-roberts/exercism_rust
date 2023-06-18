/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let mut letters = Vec::from_iter('a'..='z');
    for c in sentence.chars().flat_map(|c| c.to_lowercase()) {
        letters.retain(|l| l != &c);
        if letters.is_empty() {
            return true;
        }
    }
    letters.is_empty()
}

// alternative, less efficient.
// pub fn is_pangram(sentence: &str) -> bool {
//     ('a'..='z').all(|c| {
//         sentence
//             .chars()
//             .flat_map(|c| c.to_lowercase())
//             .any(|l| l == c)
//     })
// }
