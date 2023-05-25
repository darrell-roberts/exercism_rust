pub fn raindrops(n: u32) -> String {
    let pling = |n| (n % 3 == 0).then(|| "Pling".to_string());
    let plang = |n| (n % 5 == 0).then(|| "Plang".to_string());
    let plong = |n| (n % 7 == 0).then(|| "Plong".to_string());

    [pling(n), plang(n), plong(n)]
        .into_iter()
        .flatten()
        .reduce(|acc, s| acc + &s)
        .unwrap_or_else(|| n.to_string())
}
