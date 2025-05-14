pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
    use std::collections::HashMap;
    let mut map: HashMap<i32, usize> = HashMap::new(); //num:index

    for (i, v) in nums.iter().enumerate() {
        match map.get(v) {
            Some(j) => if ((i - j) as i32).abs() <= k { return true; },
            None => {}
        }
        map.insert(*v, i);
    }

    false
}