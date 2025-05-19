pub fn get_sneaky_numbers(nums: Vec<i32>) -> Vec<i32> {
    use std::collections::HashMap;

    nums.into_iter().fold(HashMap::new(), |mut acc, num| {
        *acc.entry(num).or_insert(0) += 1;
        acc
    }).into_iter().filter(|(k, v)| *v > 1).map(|(k, _)| k).collect::<Vec<i32>>()
}