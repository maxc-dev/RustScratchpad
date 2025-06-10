pub fn most_frequent(nums: Vec<i32>, key: i32) -> i32 {
    use std::collections::HashMap;
    let mut map = HashMap::new();
    for i in 0..nums.len()-1 {
        if (nums[i] == key) {
            *map.entry(nums[i+1]).or_insert(0) += 1;
        }
    }
    map.into_iter().max_by(|(k1, v1), (k2, v2)| v1.cmp(v2)).unwrap().0
}