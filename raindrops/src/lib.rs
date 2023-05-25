pub fn raindrops(n: u32) -> String {
    let pling = |n| (n % 3 == 0).then_some(String::from("Pling"));
    let plang = |n| (n % 5 == 0).then_some(String::from("Plang"));
    let plong = |n| (n % 7 == 0).then_some(String::from("Plong"));

    [pling(n), plang(n), plong(n)]
        .into_iter()
        .flatten()
        .reduce(|mut acc, c| {
            acc.push_str(&c);
            acc
        })
        .unwrap_or_else(|| n.to_string())
}
