/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    if isbn
        .chars()
        .rev()
        .skip(1)
        .filter(|c| c.is_ascii_digit())
        .count()
        != 9
    {
        return false;
    }

    isbn.chars()
        .filter(|c| c.is_ascii_digit() || *c == 'X')
        .flat_map(|c| if c == 'X' { Some(10) } else { c.to_digit(10) })
        .zip((1..=10).rev())
        .map(|(c, n)| c * n)
        .sum::<u32>()
        % 11
        == 0
}
