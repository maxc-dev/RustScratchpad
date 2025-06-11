pub fn frequency_sort(s: String) -> String {
    use std::collections::HashMap;
    use std::cmp::Ordering::Equal;
    let map = s.chars().into_iter().fold(HashMap::new(), |mut acc, c| {
        *acc.entry(c).or_insert(0) += 1;
        acc
    });
    let mut letters: Vec::<_> = s.chars().collect();
    letters.sort_unstable_by(|a, b| {
        match map.get(b).unwrap().cmp(map.get(a).unwrap()) {
            Equal => b.cmp(&a),
            ord => ord
        }
    });
    letters.into_iter().collect()
}