pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let upper = upper_bound as usize + 1;
    let mut values = vec![true; upper];

    for n in 2..upper {
        if values[n] {
            for i in (n..upper).step_by(n).skip_while(|i| *i <= n) {
                values[i] = false;
            }
        }
    }

    values
        .into_iter()
        .skip(2)
        .zip(2..)
        .filter_map(|(b, n)| b.then_some(n))
        .collect()
}
