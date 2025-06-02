pub fn max_freq_sum(s: String) -> i32 {
    use std::collections::{HashSet,HashMap};
    let filter: HashSet<char> = ['a', 'e', 'i', 'o', 'u'].into_iter().collect();
    let (mut con, mut vowels) = (HashMap::with_capacity(5), HashMap::with_capacity(21));

    for c in s.chars() {
        if filter.contains(&c) {
            *vowels.entry(c).or_insert(0) += 1;
        } else {
            *con.entry(c).or_insert(0) += 1;
        }
    }

    con.values().max().unwrap_or(&0) + vowels.values().max().unwrap_or(&0)
}