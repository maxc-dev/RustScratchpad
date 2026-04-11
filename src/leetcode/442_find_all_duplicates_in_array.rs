use std::collections::HashMap;

impl Solution {
    pub fn find_duplicates(nums: Vec<i32>) -> Vec<i32> {
        nums.into_iter()
            .fold(HashMap::new(), |mut acc, num| {
                *acc.entry(num).or_insert(0) += 1;
                acc
            })
            .into_iter()
            .filter_map(|(num, freq)| if freq == 2 { Some(num) } else { None })
            .collect::<Vec<_>>()
    }
}
