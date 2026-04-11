use std::collections::HashMap;

impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let n2 = nums2.len();
        let mut map = HashMap::with_capacity(n2);
        for i in 0..n2 {
            map.insert(nums2[i], i);
        }
        let mut result = Vec::with_capacity(nums1.len());
        'next: for i in nums1 {
            let index = *map.get(&i).unwrap();
            for num in nums2.iter().copied().skip(index) {
                if num > i {
                    result.push(num);
                    continue 'next;
                }
            }
            result.push(-1);
        }
        result
    }
}