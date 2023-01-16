use std::collections::BTreeSet;

/// `Palindrome` is a newtype which only exists when the contained value is a palindrome number in base ten.
///
/// A struct with a single field which is used to constrain behavior like this is called a "newtype", and its use is
/// often referred to as the "newtype pattern". This is a fairly common pattern in Rust.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Palindrome(u64);

impl Palindrome {
    /// Create a `Palindrome` only if `value` is in fact a palindrome when represented in base ten. Otherwise, `None`.
    pub fn new(value: u64) -> Option<Palindrome> {
        let value_string = value.to_string();
        let reversed_value: String = value_string.chars().rev().collect();

        match value_string == reversed_value {
            true => Some(Palindrome(value)),
            false => None,
        }
    }

    /// Get the value of this palindrome.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    if min > max {
        return None;
    }

    let mut palindromes = BTreeSet::new();

    for (index, num) in (min..=max).enumerate() {
        let inner_range = (min..=max).skip(index);
        for num_num in inner_range {
            let product = num * num_num;
            if let Some(palindrome) = Palindrome::new(product) {
                palindromes.insert(palindrome);
            }
        }
    }
    let smallest_palindrome = palindromes.iter().next()?;
    let largest_palindrome = palindromes.iter().last()?;
    Some((
        smallest_palindrome.to_owned(),
        largest_palindrome.to_owned(),
    ))
}
