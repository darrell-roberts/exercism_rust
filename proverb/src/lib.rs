pub fn build_proverb(list: &[&str]) -> String {
    let mut p = list
        .windows(2)
        .map(|s| format!("For want of a {} the {} was lost.\n", s[0], s[1]))
        .collect::<String>();

    if !list.is_empty() {
        p.push_str(&format!("And all for the want of a {}.", list[0]));
    }
    p
}
