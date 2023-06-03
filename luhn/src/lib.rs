/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    code.chars()
        .rev()
        .filter(|c| !c.is_whitespace())
        .map(|c| c.to_digit(10))
        .collect::<Option<Vec<_>>>()
        .and_then(|mut digits| {
            (digits.len() > 1).then(|| {
                for n in digits.iter_mut().skip(1).step_by(2) {
                    let val = *n * 2;
                    *n = if val > 9 { val - 9 } else { val }
                }
                digits.into_iter().sum::<u32>() % 10 == 0
            })
        })
        .unwrap_or(false)
}
