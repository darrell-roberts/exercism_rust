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
    struct Result {
        palindrome_high: Option<Palindrome>,
        palindrome_low: Option<Palindrome>,
        prod_min: u64,
        prod_max: u64,
    }

    let Result {
        palindrome_low,
        palindrome_high,
        ..
    } = (min..=max)
        .flat_map(|x| (x..=max).map(move |y| x * y))
        .fold(
            Result {
                palindrome_high: None,
                palindrome_low: None,
                prod_min: u64::MAX,
                prod_max: u64::MIN,
            },
            |mut r, prod| {
                if !(prod > r.prod_min && prod < r.prod_max) {
                    match Palindrome::new(prod) {
                        Some(p) if r.prod_min > prod => {
                            r.prod_min = prod;
                            r.palindrome_low = Some(p);
                        }
                        Some(p) if r.prod_max < prod => {
                            r.prod_max = prod;
                            r.palindrome_high = Some(p)
                        }
                        _ => (),
                    }
                }
                r
            },
        );

    palindrome_low.zip(palindrome_high)
}
