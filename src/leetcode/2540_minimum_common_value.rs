pub fn get_common(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let (mut p1, mut p2) = (0 ,0);
    let n1 = nums1.len();
    let n2 = nums2.len();

    while p1 < n1 && p2 < n2 {
        if nums1[p1] == nums2[p2] {
            return nums1[p1];
        } else if nums1[p1] < nums2[p2] {
            p1 += 1;
        } else {
            p2 += 1;
        }
    }

    -1
}