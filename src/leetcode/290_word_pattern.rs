pub fn word_pattern(pattern: String, s: String) -> bool {
    use std::collections::{HashMap,HashSet};
    let keys = pattern.split("").flat_map(|c| c.chars()).collect::<Vec<_>>();
    let values: Vec<String> = s.split(" ").map(String::from).collect();
    if keys.len() != values.len() {
        return false;
    }

    let map: HashMap<char, String> = keys.into_iter().zip(values.clone()).collect();

    if map.len() != map.values().collect::<HashSet<_>>().len() {
        return false;
    }

    for i in 0..pattern.len() {
        match map.get(&pattern.chars().nth(i).unwrap()) {
            Some(v) =>  if *v != values[i] { return false; },
            None => { }
        }
    }

    true
}