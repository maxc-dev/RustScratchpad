pub fn find_permutation_difference(s: String, t: String) -> i32 {
    use std::collections::HashMap;
    let map: HashMap<_, _> = s.chars().enumerate().map(|(i, c)| (c, i)).collect();

    t.chars().into_iter().enumerate().fold(0, |mut count, (i, c)| {
        count += ((map.get(&c).unwrap() - i) as i32).abs();
        count
    })
}