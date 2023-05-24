pub fn is_armstrong_number(num: u32) -> bool {
    let digit_iter = DigitIterator::from(num);

    digit_iter
        .count()
        .try_into()
        .ok()
        .and_then(|count| {
            digit_iter
                .map(|d| d.checked_pow(count))
                .collect::<Option<Vec<_>>>()
                .and_then(|digits| {
                    digits
                        .into_iter()
                        .fold(Some(0_u32), |n, total| n.and_then(|n| n.checked_add(total)))
                })
                .map(|result| result == num)
        })
        .unwrap_or(false)
}

#[derive(Clone, Copy)]
struct DigitIterator(Option<u32>);

impl Iterator for DigitIterator {
    type Item = u32;

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

impl From<u32> for DigitIterator {
    fn from(value: u32) -> Self {
        DigitIterator(value.into())
    }
}
