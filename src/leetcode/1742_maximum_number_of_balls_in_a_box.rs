pub fn count_balls(low_limit: i32, high_limit: i32) -> i32 {
    use std::collections::HashMap;
    let mut map: HashMap<i32, i32> = HashMap::with_capacity(10);
    for i in low_limit..=high_limit {
        let num: i32 = i.to_string().chars().map(|c| c.to_digit(10).unwrap() as i32).sum();
        *map.entry(num).or_insert(0) += 1;
    }
    map.values().map(|&c| c).max().unwrap()
}