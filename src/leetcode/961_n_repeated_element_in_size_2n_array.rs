impl Solution {
    pub fn repeated_n_times(nums: Vec<i32>) -> i32 {
        let mut store = vec![0; 10001];
        for i in nums {
            store[i as usize] += 1;
            if store[i as usize] > 1 {
                return i;
            }
        }
        -1
    }
}

// not my code but a good idea because at least one element will be a continuous sequence
impl Solution {
    // Complexity:
    // Time O(N) and Space O(1) where N is the length of nums.
    pub fn repeated_n_times(nums: Vec<i32>) -> i32 {
        for i in 1..nums.len() {
            if nums[i] == nums[i - 1] {
                return nums[i];
            }
        }

        if nums[1] == nums[3] { nums[1] } else { nums[0] }
    }
}