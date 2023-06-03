use std::collections::{HashMap, HashSet};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word_map: HashMap<char, u8> =
        word.chars()
            .flat_map(char::to_lowercase)
            .fold(HashMap::new(), |mut acc, c| {
                *acc.entry(c).or_insert(0) += 1;
                acc
            });

    possible_anagrams
        .iter()
        .filter(|&w| {
            *w.to_lowercase() != word.to_lowercase() && is_anagram(&word_map, &w.to_lowercase())
        })
        .cloned()
        .collect()
}

fn is_anagram(set: &HashMap<char, u8>, word: &str) -> bool {
    let word_map = word
        .chars()
        .flat_map(char::to_lowercase)
        .fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c).or_insert(0) += 1;
            acc
        });

    word_map
        .iter()
        .all(|(key, value)| set.get(key) == Some(value)) && word_map.len() >= set.len()
}
