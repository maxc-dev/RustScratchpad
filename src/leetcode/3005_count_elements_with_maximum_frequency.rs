pub fn max_frequency_elements(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;
    let map = nums.into_iter().fold(HashMap::new(), |mut acc, num| {
        *acc.entry(num).or_insert(0) += 1;
        acc
    });
    let max = map.values().max().unwrap();
    map.values().filter(|v| *v == max).count() as i32 * max
}