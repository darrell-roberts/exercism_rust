//! A digit iterator that can yield digits forwards
//! or backwards from an intial u64 value.

#[must_use]
#[derive(Clone, Copy)]
/// An iterator that yields digits.
pub struct DigitIterator {
    value: Option<u64>,
    divisor: u64,
    total_digits: usize,
}

impl DigitIterator {
    /// Create a digit iterator from a u64.
    pub fn new(value: u64) -> Self {
        let mut divisor = 1;
        let mut total_digits = 1;
        while (value / divisor) >= 10 {
            divisor *= 10;
            total_digits += 1;
        }

        Self {
            value: Some(value),
            divisor,
            total_digits,
        }
    }
}

impl Iterator for DigitIterator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.value.take().map(|n| {
            let digit = n / self.divisor;
            let result = n % self.divisor;
            if result > 0 {
                self.value = Some(result);
                self.divisor /= 10;
            }
            self.total_digits -= 1;
            digit
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.total_digits, Some(self.total_digits))
    }
}

impl DoubleEndedIterator for DigitIterator {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.value.take().map(|n| {
            let result = n / 10;
            if result > 0 {
                self.value = Some(result);
            }
            self.total_digits -= 1;
            n % 10
        })
    }
}

impl ExactSizeIterator for DigitIterator {}
