use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn two_out_of_three(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>) -> Vec<i32> {
        let mut map = HashMap::new();
        let mut count_freq = |mut nums: Vec<i32>| {
            for n in nums.into_iter().collect::<HashSet<_>>().into_iter() {
                *map.entry(n).or_insert(0) += 1;
            }
        };

        count_freq(nums1);
        count_freq(nums2);
        count_freq(nums3);

        map.into_iter()
            .filter_map(|(num, count)| if count >= 2 { Some(num) } else { None })
            .collect()
    }
}

// not my solution but a great idea using bit manipulation
impl Solution {
    pub fn two_out_of_three(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>) -> Vec<i32> {
        let mut counter = [0_i32; 101];
        nums1.iter().for_each(|&x| counter[x as usize] |= 0b001);
        nums2.iter().for_each(|&x| counter[x as usize] |= 0b010);
        nums3.iter().for_each(|&x| counter[x as usize] |= 0b100);
        counter
            .iter()
            .enumerate()
            .filter_map(|(ind, x)| if x.count_ones() >= 2 { Some(ind as i32) } else { None })
            .collect::<Vec<_>>()
    }
}