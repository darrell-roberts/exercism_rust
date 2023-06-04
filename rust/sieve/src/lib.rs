pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let upper = upper_bound as usize;
    let mut values = vec![true; upper + 1];

    for i in (2..=upper).flat_map(|n| (n..=upper).step_by(n).skip_while(move |i| *i <= n)) {
        values[i] = false;
    }

    values
        .into_iter()
        .skip(2)
        .zip(2..)
        .filter_map(|(b, n)| b.then_some(n))
        .collect()
}
