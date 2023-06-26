use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    h.iter()
        .flat_map(|(score, chars)| {
            chars
                .iter()
                .flat_map(|c| c.to_lowercase())
                .map(|c| (c, *score))
        })
        .collect()
}
