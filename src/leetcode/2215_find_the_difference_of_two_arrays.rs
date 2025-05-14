pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
    use std::collections::HashSet;
    let mut set1 = nums1.into_iter().collect::<HashSet<i32>>();
    let mut set2 = nums2.into_iter().collect::<HashSet<i32>>();

    let unique1: Vec<i32> = set1.difference(&set2).map(|&c| c).collect();
    let unique2: Vec<i32> = set2.difference(&set1).map(|&c| c).collect();

    vec![unique1, unique2]
}