pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    for i in m..m+n {
        nums1[i as usize] = nums2[(i-m) as usize];
    }
    nums1.sort();
}