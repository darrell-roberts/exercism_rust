use std::collections::HashSet;

fn sorted_chars(word: &str) -> Vec<char> {
    let mut lowered = word
        .chars()
        .flat_map(|c| c.to_lowercase())
        .collect::<Vec<_>>();
    lowered.sort_unstable();
    lowered
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let target_sorted_chars = sorted_chars(word);

    let same_word = |word1: &str, word2: &str| {
        word1
            .chars()
            .flat_map(|c| c.to_lowercase())
            .eq(word2.chars().flat_map(|c| c.to_lowercase()))
    };

    possible_anagrams
        .iter()
        .copied()
        .filter(|t_word| t_word.len() == word.len())
        .filter(|t_word| !same_word(t_word, word))
        .filter(|t_word| sorted_chars(t_word) == target_sorted_chars)
        .collect::<HashSet<_>>()
}
