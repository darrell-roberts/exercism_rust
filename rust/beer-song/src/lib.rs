const B_O_B: &str = "bottles of beer";
const O_T_W: &str = "on the wall";
const T_O_D: &str = "Take one down and pass it around,";
const A_B_O_B: &str = "bottle of beer";

pub fn verse(n: u32) -> String {
    match n {
        0 => format!("No more {B_O_B} {O_T_W}, no more {B_O_B}.\nGo to the store and buy some more, 99 {B_O_B} {O_T_W}.\n"),
        1 => format!("1 {A_B_O_B} {O_T_W}, 1 {A_B_O_B}.\nTake it down and pass it around, no more {B_O_B} {O_T_W}.\n"),
        2 => format!("{n} {B_O_B} {O_T_W}, {n} {B_O_B}.\n{T_O_D} 1 {A_B_O_B} {O_T_W}.\n"),
        n => format!(
            "{n} {B_O_B} {O_T_W}, {n} {B_O_B}.\n{T_O_D} {} {B_O_B} {O_T_W}.\n",
            n - 1
        ),
    }
}

pub fn sing(start: u32, end: u32) -> String {
    (end..=start)
        .map(|n| (n, verse(n)))
        .rev()
        .fold(String::new(), |mut s, (i, v)| {
            s.push_str(&v);
            if i != end {
                s.push('\n');
            }
            s
        })
}
