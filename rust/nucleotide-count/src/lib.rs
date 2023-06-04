use std::{collections::HashMap, matches};

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    let initial_count = matches!(nucleotide, 'A' | 'T' | 'C' | 'G')
        .then_some(0)
        .ok_or(nucleotide);

    dna.chars().fold(initial_count, |acc, c| {
        acc.and_then(|count| match c {
            'A' | 'T' | 'C' | 'G' => Ok(if c == nucleotide { count + 1 } else { count }),
            ch => Err(ch),
        })
    })
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut counts = HashMap::from([('A', 0), ('T', 0), ('C', 0), ('G', 0)]);

    for c in dna.chars() {
        counts.get_mut(&c).map(|count| *count += 1).ok_or(c)?
    }
    Ok(counts)
}
