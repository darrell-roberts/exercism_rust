use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let multiples = |&n| {
        std::iter::once(n)
            .chain((2..).map(move |i| i * n))
            .take_while(|&i| i < limit && i > 0)
    };

    factors
        .iter()
        .flat_map(multiples)
        .collect::<HashSet<_>>()
        .into_iter()
        .sum()
}
