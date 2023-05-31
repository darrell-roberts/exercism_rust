use std::collections::HashMap;
use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    if worker_count == 1 {
        input
            .iter()
            .flat_map(|s| s.chars())
            .filter(|c| c.is_alphabetic())
            .flat_map(|c| c.to_lowercase())
            .fold(HashMap::new(), |mut counts, c| {
                counts.entry(c).and_modify(|n| *n += 1).or_insert(1);
                counts
            })
    } else {
        let one_string = input.join("");
        let batch_size = (one_string.len() / worker_count) + 1;
        let mut char_iter = one_string.chars();

        (0..worker_count)
            .map(|_| {
                let batch = char_iter.by_ref().take(batch_size).collect::<String>();
                thread::spawn(move || {
                    batch
                        .chars()
                        .filter(|c| c.is_alphabetic())
                        .flat_map(|c| c.to_lowercase())
                        .fold(HashMap::new(), |mut counts, c| {
                            counts.entry(c).and_modify(|n| *n += 1).or_insert(1);
                            counts
                        })
                })
            })
            .collect::<Vec<_>>()
            .into_iter()
            .fold(HashMap::new(), |mut result, worker| {
                let m = worker.join().unwrap();
                for (k, v) in m {
                    result.entry(k).and_modify(|n| *n += v).or_insert(v);
                }
                result
            })
    }
}
