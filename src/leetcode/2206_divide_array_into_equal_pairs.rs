pub fn divide_array(nums: Vec<i32>) -> bool {
    use std::collections::HashMap;
    nums.into_iter().fold(HashMap::new(), |mut acc, num| {
        *acc.entry(num).or_insert(0) += 1;
        acc
    }).values().all(|v| *v % 2 == 0)
}