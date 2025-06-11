pub fn number_of_pairs(nums: Vec<i32>) -> Vec<i32> {
    use std::collections::HashMap;
    nums.into_iter().fold(HashMap::new(), |mut acc, num| {
        *acc.entry(num).or_insert(0) += 1;
        acc
    }).into_iter().fold(vec![0, 0], |mut acc, (_, freq)| {
        let count_pairs = freq / 2;
        acc[0] += count_pairs;
        acc[1] += freq % 2;
        acc
    })
}