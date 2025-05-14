pub fn intersection(nums: Vec<Vec<i32>>) -> Vec<i32> {
    use std::collections::HashSet;
    let mut start = nums[0].clone().into_iter().collect::<HashSet<i32>>();
    for arr in nums {
        start = start.intersection(&arr.into_iter().collect::<HashSet<i32>>())
            .map(|&v| v)
            .collect::<HashSet<i32>>();
    }
    let mut res = start.into_iter().collect::<Vec<_>>();
    res.sort();
    res
}