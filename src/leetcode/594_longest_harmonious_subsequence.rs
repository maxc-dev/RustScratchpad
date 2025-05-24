pub fn find_lhs(nums: Vec<i32>) -> i32 {
    use std::cmp::max;
    let mut nums = nums;
    nums.sort();
    let (mut count, mut res) = (0, 0);
    let (mut p1, mut p2) = (0, 1);

    while p2 < nums.len() {
        let diff = (nums[p1] - nums[p2]).abs();
        if (diff == 1) {
            p2 += 1;
            count = max(p2 - p1, count);
        } else if (diff == 0) {
            p2 += 1;
            res = max(p2 - p1, res);
        } else {
            if ((nums[p2-1] - nums[p1]).abs()) == 1 {
                count = max(count, res);
            }
            p1 += 1;
            p2 = p1 + 1;
            res = 0;
        }
    }

    count as i32
} // todo next time consider hash map + fold