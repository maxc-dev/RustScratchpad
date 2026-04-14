impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        fn solve(index: usize, current: &mut Vec<i32>, nums: &Vec<i32>, res: &mut Vec<Vec<i32>>) {
            if index == nums.len() {
                res.push(current.clone());
                return;
            }
            current.push(nums[index]);
            solve(index + 1, current, nums, res);
            current.pop();
            solve(index + 1, current, nums, res);
        }

        solve(0, &mut Vec::new(), &nums, &mut res);
        res
    }
}