use itertools::Itertools;
use std::collections::HashMap;

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    // Get unique characters
    let mut unique_chars: Vec<_> = input.chars().filter(char::is_ascii_alphabetic).collect();
    unique_chars.sort();
    unique_chars.dedup();

    //Generate unique permutations of digits (0 - 9)
    let permutation_len = unique_chars.len();
    let permutations = (0..=9).permutations(permutation_len).unique();

    // Split equation into parts
    let mut split = input.split("==");
    let lhs: Vec<&str> = split.next()?.split('+').map(|x| x.trim()).collect();
    let rhs: &str = split.next()?.trim();

    //Handle edge cases
    if lhs.iter().any(|x| x.len() > rhs.len()) || unique_chars.len() > 10 {
        return None;
    }

    //Iterate through each permutation
    //testing evaluation of equation equality
    for permutation in permutations {
        let digit_map: HashMap<char, u8> = unique_chars.iter().cloned().zip(permutation).collect();
        if lhs.iter().any(|w| digit_map[&w.chars().next().unwrap()] == 0u8) || digit_map[&rhs.chars().next().unwrap()] == 0u8 { continue; };

        let lhs: usize = lhs
            .iter()
            .map(|x| {
                x.chars()
                    .map(|c| digit_map.get(&c).unwrap())
                    .join("")
                    .parse::<usize>()
                    .unwrap()
            })
            .sum();
        let rhs = rhs
            .chars()
            .map(|x| digit_map.get(&x).unwrap())
            .join("")
            .parse::<usize>()
            .unwrap();

        if lhs == rhs {
            dbg!(&digit_map);
            println!("Solution found");
            return Some(digit_map);
        }
    }

    None
}
