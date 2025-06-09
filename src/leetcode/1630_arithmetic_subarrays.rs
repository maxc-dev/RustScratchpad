pub fn check_arithmetic_subarrays(nums: Vec<i32>, l: Vec<i32>, r: Vec<i32>) -> Vec<bool> {
    let subarr = |arr: &[i32]| -> bool {
        let mut vec = arr.to_vec();
        vec.sort_unstable();
        let diff = (vec[0] - vec[1]).abs();
        vec.windows(2).all(|w| (w[1] - w[0]).abs() == diff)
    };

    l.into_iter()
        .zip(r.into_iter())
        .map(|(start, end)| subarr(&nums[start as usize..=end as usize]))
        .collect()
}