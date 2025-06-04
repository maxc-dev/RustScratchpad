pub fn are_occurrences_equal(s: String) -> bool {
    use std::collections::HashMap;
    s.chars().into_iter().fold(HashMap::new(), |mut acc, c| {
        *acc.entry(c).or_insert(0) += 1;
        acc
    }).values().collect::<Vec<_>>().windows(2).all(|w| w[0] == w[1])
}