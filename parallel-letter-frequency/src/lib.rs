use std::collections::HashMap;
use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    match input.len() {
        0 => HashMap::new(),
        x if x < 300 => calculate_letter_frequency(input),
        x => thread::scope(|s| {
            let mut map = HashMap::new();
            let mut thread_pool = Vec::with_capacity(worker_count);
            for chunk in input.chunks(x / worker_count + 1) {
                thread_pool.push(s.spawn(|| calculate_letter_frequency(chunk)));
            }
            for thread in thread_pool {
                thread.join().unwrap().iter().for_each(|(&ch, &count)| {
                    map.entry(ch).and_modify(|c| *c += count).or_insert(count);
                });
            }
            map
        }),
    }
}

fn calculate_letter_frequency(input: &[&str]) -> HashMap<char, usize> {
    input
        .iter()
        .flat_map(|&line| line.chars())
        .filter(|ch| ch.is_alphabetic())
        .filter_map(|ch| ch.to_lowercase().next())
        .fold(HashMap::new(), |mut map, letter| {
            *map.entry(letter).or_default() += 1;
            map
        })
}
