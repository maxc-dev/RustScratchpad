impl Solution {
    pub fn zero_filled_subarray(nums: Vec<i32>) -> i64 {
        let mut count = 0;
        let mut current = 0;
        for num in nums.into_iter() {
            if num == 0 {
                current += 1;
                count += current;
            } else {
                current = 0;
            }
        }
        count
    }
}