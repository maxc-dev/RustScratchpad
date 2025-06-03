pub fn arithmetic_triplets(nums: Vec<i32>, diff: i32) -> i32 {
    use std::collections::HashSet;
    let set = nums.clone().into_iter().collect::<HashSet<_>>();
    nums.into_iter().fold(0, |mut acc, i| {
        if set.contains(&(i + diff)) && set.contains(&(i + (diff * 2))) {
            acc += 1;
        }
        acc
    })
}