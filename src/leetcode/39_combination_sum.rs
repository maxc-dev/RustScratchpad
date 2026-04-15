impl Solution {
    pub fn combination_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        fn solve(current: &[i32], target: i32, nums: &[i32], res: &mut Vec<Vec<i32>>) {
            let sum = current.iter().sum::<i32>();
            if sum >= target {
                if sum == target {
                    res.push(current.to_vec());
                }
                return;
            }

            for (i, v) in nums.iter().enumerate() {
                let mut current = current.to_vec();
                current.push(*v);
                solve(&current, target, &nums[i..], res);
            }
        }

        let mut res = Vec::new();
        solve(&vec![], target, &nums, &mut res);
        res
    }
}