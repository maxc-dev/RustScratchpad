impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let nums = (1..=n).into_iter().collect::<Vec<i32>>();

        fn solve(index: usize, n: i32, k: i32, current: &mut Vec<i32>, nums: &Vec<i32>, res: &mut Vec<Vec<i32>>) {
            if current.len() == k as usize {
                res.push(current.clone());
                return;
            }
            for i in index..n as usize {
                current.push(nums[i]);
                solve(i + 1, n, k, current, nums, res);
                current.pop();
            }
        }

        let mut res = Vec::with_capacity(n as usize * k as usize);
        solve(0, n, k, &mut vec![], &nums, &mut res);
        res
    }
}