pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split(|c: char| c.is_whitespace() || (!c.is_alphabetic() && c != '\''))
        .flat_map(|word| CamelCaseSplitter { word: Some(word) })
        .flat_map(|word| word.chars().next())
        .flat_map(|c| c.to_uppercase())
        .collect()
}

// pub fn abbreviate(phrase: &str) -> String {
//     phrase
//         .split(|c: char| c.is_ascii_whitespace() || c == '_' || c == '-')
//         .flat_map(|word| {
//             word.chars().take(1).chain(
//                 word.chars()
//                     .skip_while(|c| c.is_ascii_uppercase())
//                     .filter(|c| c.is_ascii_uppercase()),
//             )
//         })
//         .collect::<String>()
//         .to_ascii_uppercase()
// }

struct CamelCaseSplitter<'a> {
    word: Option<&'a str>,
}

impl<'a> Iterator for CamelCaseSplitter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let next_upper = |s: &'a str| {
            s.chars()
                .skip(1)
                .position(|c| c.is_uppercase())
                .map(|p| s.split_at(p + 1))
        };

        self.word.take().and_then(|w| {
            let has_camel_case = w.chars().skip(1).any(|c| c.is_uppercase());
            let all_caps = w.chars().all(|c| c.is_uppercase());
            (!has_camel_case || all_caps).then_some(w).or_else(|| {
                next_upper(w)
                    .map(|(one, two)| {
                        self.word = Some(two);
                        one
                    })
                    .or(Some(w))
            })
        })
    }
}
