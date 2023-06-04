fn is_prime(n: &u32) -> bool {
    match n {
        0 | 1 => false,
        2 | 3 => true,
        n if n % 2 == 0 || n % 3 == 0 => false,
        n => {
            let end = 1 + ((*n as f32).sqrt() as u32);
            !(5..=end).step_by(6).any(|i| n % i == 0 || n % (i + 2) == 0)
        }
    }
}

pub fn nth(n: u32) -> u32 {
    (0..)
        .filter(is_prime)
        .zip(0_u32..)
        .find(|&(_, index)| index == n)
        .map(|(p, _)| p)
        .unwrap_or_default()
}
