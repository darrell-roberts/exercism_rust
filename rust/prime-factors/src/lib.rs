struct FactorsIter(u64);

impl Iterator for FactorsIter {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        (self.0 > 1).then_some(2).and_then(|n| {
            (n..).find_map(|d| {
                (self.0 % d == 0).then(|| {
                    self.0 /= d;
                    d
                })
            })
        })
    }
}

pub fn factors(n: u64) -> Vec<u64> {
    FactorsIter(n).collect()
}
