pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
    use std::collections::HashSet;
    let n = (nums.len() as i32) + 1;
    let mut set: HashSet<i32> = nums.into_iter().collect::<HashSet<i32>>();

    let mut res: Vec<i32> = vec![];
    for i in 1..n {
        if !set.contains(&i) {
            res.push(i);
        }
    }
    res
}