pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    use std::collections::HashMap;
    let mut map = nums1.into_iter().fold(HashMap::new(), |mut acc, num| {
        *acc.entry(num).or_insert(0) += 1;
        acc
    });

    let mut res: Vec<i32> = Vec::new();

    for i in nums2 {
        if let Some(count) = map.get_mut(&i) {
            if (*count > 0) {
                res.push(i);
                *count -= 1;
            }
        }
    }

    res
}