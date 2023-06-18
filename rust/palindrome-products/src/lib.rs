/// `Palindrome` is a newtype which only exists when the contained value is a palindrome number in base ten.
///
/// A struct with a single field which is used to constrain behavior like this is called a "newtype", and its use is
/// often referred to as the "newtype pattern". This is a fairly common pattern in Rust.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Palindrome(u64);

impl Palindrome {
    /// Create a `Palindrome` only if `value` is in fact a palindrome when represented in base ten. Otherwise, `None`.
    pub fn new(value: u64) -> Option<Palindrome> {
        let mut divisor = value;
        let reverse = std::iter::from_fn(move || {
            (divisor != 0).then(|| {
                let (d, r) = (divisor / 10, divisor % 10);
                divisor = d;
                r
            })
        })
        .fold(0, |reverse, digit| reverse * 10 + digit);
        (value == reverse).then_some(Self(value))
    }

    /// Get the value of this palindrome.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let (highest, lowest) = (min..=max)
        .flat_map(|x| (x..=max).map(move |y| (x, y)))
        .map(|(x, y)| x * y)
        .flat_map(Palindrome::new)
        .fold((None, None), |(h, l), n| match (h, l) {
            (Some(h), Some(_)) if n > h => (Some(n), l),
            (Some(_), Some(l)) if n < l => (h, Some(n)),
            (None, None) => (Some(n), Some(n)),
            _ => (h, l),
        });
    lowest.and_then(|l| highest.map(|h| (l, h)))
}
