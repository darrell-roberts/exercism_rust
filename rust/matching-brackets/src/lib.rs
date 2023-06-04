pub fn brackets_are_balanced(string: &str) -> bool {
    let mut open_brackets = vec![];

    let closing_bracket = |c: char| {
        match c {
            '{' => '}',
            '[' => ']',
            '(' => ')',
            _ => return None,
        }
        .into()
    };

    for c in string.chars() {
        match c {
            '{' | '[' | '(' => {
                open_brackets.push(c);
            }
            '}' | ']' | ')' => {
                let matched = open_brackets
                    .pop()
                    .and_then(closing_bracket)
                    .map(|open| open == c)
                    .unwrap_or(false);

                if !matched {
                    return false;
                }
            }
            _ => (),
        }
    }

    open_brackets.is_empty()
}
