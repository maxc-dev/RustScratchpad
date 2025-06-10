pub fn frequency_sort(mut nums: Vec<i32>) -> Vec<i32> {
    use std::collections::HashMap;
    use std::cmp::Ordering::Equal;
    let mut freq = HashMap::new();
    for &num in &nums {
        *freq.entry(num).or_insert(0) += 1;
    }
    nums.sort_unstable_by(|a, b| {
        match freq.get(a).unwrap().cmp(freq.get(b).unwrap()) {
            Equal => b.cmp(&a),
            ordering => ordering
        }
    });
    nums
}