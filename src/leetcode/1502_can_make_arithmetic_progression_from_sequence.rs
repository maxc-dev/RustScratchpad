pub fn can_make_arithmetic_progression(arr: Vec<i32>) -> bool {
    let mut arr = arr;
    arr.sort_unstable();
    let diff = (arr[0] - arr[1]).abs();
    arr.windows(2).all(|w| (w[1] - w[0]).abs() == diff)
}