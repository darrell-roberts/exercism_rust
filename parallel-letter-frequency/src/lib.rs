use std::collections::HashMap;
use std::thread;

fn count_chars(words: Vec<String>) -> HashMap<char, usize> {
    words
        .iter()
        .flat_map(move |s| {
            s.chars()
                .filter(|c| c.is_alphabetic())
                .flat_map(move |c| c.to_lowercase())
        })
        .fold(HashMap::new(), |mut counts, c| {
            counts.entry(c).and_modify(|n| *n += 1).or_insert(1);
            counts
        })
}

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let batch_size = {
        let size = input.len() / worker_count;
        if size > 0 {
            size
        } else {
            1
        }
    };

    input
        .chunks(batch_size)
        .map(|batch| batch.iter().map(|s| s.to_string()).collect::<Vec<_>>())
        .map(|batch| thread::spawn(move || count_chars(batch)))
        .fold(HashMap::new(), |mut result, worker| {
            let m = worker.join().unwrap();
            for (k, v) in m {
                result.entry(k).and_modify(|n| *n += v).or_insert(v);
            }
            result
        })
}
