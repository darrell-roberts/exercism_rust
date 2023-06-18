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
        // Using this iterator is a bit slower vs
        // the in place algorithm commented out below.
        // ~ 7 seconds.
        let digit_iter = DigitIterator::new(value);
        let half = digit_iter.len() / 2;

        digit_iter
            .take(half)
            .eq(digit_iter.rev().take(half))
            .then_some(Self(value))

        // A faster algorithm. ~ 3 seconds.

        // let mut result = value;
        // let mut divisor = 1;
        // while (value / divisor) >= 10 {
        //     divisor *= 10;
        // }

        // while result != 0 {
        //     let leading = result / divisor;
        //     let trailing = result % 10;
        //     if leading != trailing {
        //         return None;
        //     }
        //     result = (result % divisor) / 10;
        //     divisor /= 100;
        // }
        // Some(Self(value))
    }

    /// Get the value of this palindrome.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let (highest, lowest) = (min..=max)
        .filter(|n| n % 10 != 0)
        .flat_map(|x| (x..=max).filter(|n| n % 10 != 0).map(move |y| (x, y)))
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
