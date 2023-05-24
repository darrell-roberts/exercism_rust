use std::panic;

pub fn square(s: u32) -> u64 {
    if s > 64 || s == 0 {
        panic!("Square must be between 1 and 64")
    }
    (1_u64..s.into()).fold(1, |acc, _| acc * 2)
}

pub fn total() -> u64 {
    (1..=64).map(square).sum()
}
