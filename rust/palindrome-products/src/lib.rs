use digit::DigitIterator;

mod digit;

/// `Palindrome` is a newtype which only exists when the contained value is a palindrome number in base ten.
///
/// A struct with a single field which is used to constrain behavior like this is called a "newtype", and its use is
/// often referred to as the "newtype pattern". This is a fairly common pattern in Rust.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Palindrome(u64);

impl Palindrome {
    /// Create a `Palindrome` only if `value` is in fact a palindrome when represented in base ten. Otherwise, `None`.
    pub fn new(value: u64) -> Option<Palindrome> {
        let digit_iter = DigitIterator::new(value);
        let half = digit_iter.len() / 2;

        digit_iter
            .take(half)
            .eq(digit_iter.rev().take(half))
            .then_some(Self(value))
    }

    /// Get the value of this palindrome.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut prod_min = u64::MAX;
    let mut prod_max = u64::MIN;

    let (lowest, highest) = (min..=max)
        .flat_map(|x| (x..=max).map(move |y| x * y))
        .fold((None, None), |(l, h), prod| {
            if prod > prod_min && prod < prod_max {
                return (l, h);
            }

            match Palindrome::new(prod) {
                Some(p) if prod_min > prod => {
                    prod_min = prod;
                    (Some(p), h)
                }
                Some(p) if prod_max < prod => {
                    prod_max = prod;
                    (l, Some(p))
                }
                _ => (l, h),
            }
        });

    lowest.zip(highest)
}
