pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    use std::collections::HashSet;
    nums1.into_iter().collect::<HashSet<i32>>()
        .intersection(&nums2.into_iter().collect::<HashSet<i32>>())
        .map(|&v| v).collect()
}