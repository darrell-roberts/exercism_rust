use std::ops::Not;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return an `Err(.)` if the conversion is impossible.
/// The tests do not test for specific values inside the `Err(.)`.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits, unless the input number is 0, in which the output must be `[0]`.
///    However, your function must be able to process input with leading 0 digits.
///
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    // Check our input.
    if let Some(err) = number
        .iter()
        .find(|&n| *n >= from_base)
        .map(|n| Error::InvalidDigit(*n))
        .or_else(|| (from_base < 2).then_some(Error::InvalidInputBase))
        .or_else(|| (to_base < 2).then_some(Error::InvalidOutputBase))
    {
        return Err(err);
    }

    // Compute input to a number using from_base.
    let n = number
        .iter()
        .rev()
        .zip(0..)
        .map(|(n, i)| n * (from_base.pow(i)))
        .sum::<u32>();

    // Compute number to output using to_base.
    let mut last_div = n;
    let mut last_digit_reached = false;
    let result = std::iter::from_fn(move || {
        last_digit_reached.not().then(|| {
            let (d, last_mod) = (last_div / to_base, last_div % to_base);
            if d == 0 {
                last_digit_reached = true;
            }
            last_div = d;
            last_mod
        })
    })
    .collect::<Vec<_>>();

    Ok(result.into_iter().rev().collect())
}
