pub fn minimum_abs_difference(arr: Vec<i32>) -> Vec<Vec<i32>> {
    use std::cmp::min;
    let mut j = i32::MAX;
    let mut arr = arr;
    arr.sort_unstable();

    for i in 0..arr.len()-1 {
        j = min(j, (arr[i] - arr[i+1]).abs());
    }

    let mut vec: Vec<Vec<i32>> = Vec::new();
    for i in 0..arr.len()-1 {
        if (arr[i] - arr[i+1]).abs() <= j {
            vec.push(vec![arr[i], arr[i+1]]);
        }
    } // todo this could be done in one sweep ...
    vec
}