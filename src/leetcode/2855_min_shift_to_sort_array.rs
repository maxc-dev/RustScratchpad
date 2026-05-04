impl Solution {
    pub fn minimum_right_shifts(nums: Vec<i32>) -> i32 {
        let mut low_val = nums[0];
        let mut low_idx = 0usize;
        let mut high_val = i32::MIN;
        let mut high_idx = 0usize;
        let N = nums.len();
        for (idx, val) in nums.iter().enumerate() {
            if *val < low_val {
                low_val = *val;
                low_idx = idx;
            }
            if *val > high_val {
                high_val = *val;
                high_idx = idx;
            }
        }

        if (high_idx + 1) % N != low_idx % N {
            return -1;
        }

        for i in 1..N {
            if i > low_idx {
                if nums[i] < nums[i-1] {
                    return -1;
                }
            } else if i < high_idx {
                if nums[i] < nums[i-1] {
                    return -1;
                }
            }
        }

        let shift = N as i32 - high_idx as i32 - 1;
        if shift != 0 && nums[N-1] > nums[0] {
            return -1;
        }
        shift
    }
}