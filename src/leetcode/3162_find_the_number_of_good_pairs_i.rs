pub fn number_of_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i32 {
    let pairs = nums2.into_iter().map(|i| i * k).collect::<Vec<_>>();
    let mut count = 0;
    for a in nums1 {
        for &b in &pairs {
            if a % b == 0 {
                count += 1;
            }
        }
    }
    count
}