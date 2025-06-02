pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
    use std::collections::HashSet;
    let allowed = allowed.chars().collect::<HashSet<_>>();
    words.into_iter().filter(|s| s.chars().all(|c| allowed.contains(&c))).count() as i32
}