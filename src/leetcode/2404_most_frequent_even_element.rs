pub fn most_frequent_even(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;
    let (mut max_k, mut max_c) = (-1, 0);
    let map = nums.into_iter().fold(HashMap::new(), |mut acc, num| {
        if num % 2 == 0 {
            let count_ref = acc.entry(num).or_insert(0);
            *count_ref += 1;
            if *count_ref > max_c || (*count_ref == max_c && num < max_k) {
                max_k = num;
                max_c = *count_ref;
            }
        }
        acc
    });
    max_k
}