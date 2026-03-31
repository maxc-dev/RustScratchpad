pub fn alternating_sum(nums: Vec<i32>) -> i32 {
    let mut res: i32 = 0;
    for (i, &num) in nums.iter().enumerate().step_by(2) {
        res = res + num;
    }
    for (i, &num) in nums.iter().enumerate().skip(1).step_by(2) {
        res = res - num;
    }
    res
}