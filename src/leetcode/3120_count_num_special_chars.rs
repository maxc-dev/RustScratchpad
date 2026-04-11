use std::collections::HashSet;

impl Solution {
    pub fn number_of_special_chars(word: String) -> i32 {
        let mut upper = HashSet::new();
        let mut lower = HashSet::new();

        for c in word.chars() {
            if c.is_ascii_uppercase() {
                upper.insert(c);
            } else if c.is_ascii_lowercase() {
                lower.insert(c);
            }
        }

        lower
            .into_iter()
            .filter(|c| upper.contains(&c.to_ascii_uppercase()))
            .count() as i32
    }
}