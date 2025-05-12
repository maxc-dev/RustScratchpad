pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
    let mut strings: Vec<String> = Vec::new();
    if nums.is_empty() {
        return strings;
    }
    let mut first = nums[0];
    let mut current = nums[0];

    let format_range = |start: i32, end: i32| {
        if start == end {
            start.to_string()
        } else {
            format!("{}->{}", start, end)
        }
    };

    for i in 1..nums.len() {
        if nums[i] - 1 != nums[i-1] {
            strings.push(format_range(first, current));
            first = nums[i];
        }
        current = nums[i];
    }
    strings.push(format_range(first, current));
    strings
}