use std::collections::HashSet;

/// `Palindrome` is a newtype which only exists when the contained value is a palindrome number in base ten.
///
/// A struct with a single field which is used to constrain behavior like this is called a "newtype", and its use is
/// often referred to as the "newtype pattern". This is a fairly common pattern in Rust.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Palindrome(u64);

impl Palindrome {
    /// Create a `Palindrome` only if `value` is in fact a palindrome when represented in base ten. Otherwise, `None`.
    pub fn new(value: u64) -> Option<Palindrome> {
        // let s = DigitIterator(Some(value)).collect::<Vec<_>>();
        // let half = s.len() / 2;
        // s.iter()
        //     .take(half)
        //     .eq(s.iter().rev().take(half))
        //     .then_some(Self(value))
        let s = value.to_string();
        let half = s.len() / 2;
        s.chars()
            .take(half)
            .eq(s.chars().rev().take(half))
            .then_some(Self(value))
    }

    /// Get the value of this palindrome.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let start = std::time::Instant::now();
    println!("computing totals");
    let totals = (min..=max)
        .flat_map(|x| (x..=max).map(move |y| (x, y)))
        // .inspect(|(x, y)| println!("({x}, {y})"))
        .map(|(x, y)| x * y)
        .collect::<HashSet<_>>();

    println!(
        "{} totals in {} seconds",
        totals.len(),
        start.elapsed().as_secs()
    );
    let start = std::time::Instant::now();
    let (highest, lowest) =
        totals
            .into_iter()
            .flat_map(Palindrome::new)
            .fold((None, None), |(h, l), n| match (h, l) {
                (Some(h), Some(_)) if n > h => (Some(n), l),
                (Some(_), Some(l)) if n < l => (h, Some(n)),
                (None, None) => (Some(n), Some(n)),
                _ => (h, l),
            });
    println!("palindromes in {} seconds", start.elapsed().as_secs());
    lowest.and_then(|l| highest.map(|h| (l, h)))
}

#[derive(Clone, Copy)]
struct DigitIterator(Option<u64>);

impl Iterator for DigitIterator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.take().map(|n| {
            let result = n / 10;
            if result > 0 {
                self.0 = result.into();
            }
            n % 10
        })
    }
}
