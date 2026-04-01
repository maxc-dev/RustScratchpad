pub fn separate_digits(nums: Vec<i32>) -> Vec<i32> {
    nums.into_iter()
        .flat_map(|i| {
            i.to_string()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect::<Vec<_>>()
        })
        .collect()
}
