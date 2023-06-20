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
        // Time (mean ± σ):       28.6 ms ±   0.8 ms    [User: 53.6 ms, System: 1.5 ms]
        // Range (min … max):    27.0 ms …  31.4 ms   100 runs
        let digit_iter = DigitIterator::new(value);
        let half = digit_iter.len() / 2;

        digit_iter
            .take(half)
            .eq(digit_iter.rev().take(half))
            .then_some(Self(value))

        // Time (mean ± σ):       26.0 ms ±   1.5 ms    [User: 47.9 ms, System: 1.4 ms]
        // Range (min … max):    24.5 ms …  34.9 ms   100 runs

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
