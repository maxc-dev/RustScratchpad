impl Solution {
    pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        grid.into_iter()
            .map(|arr| {
                let mut count = 0;
                for i in (0..arr.len()).rev() {
                    if arr[i] < 0 {
                        count += 1;
                    } else {
                        break;
                    }
                }
                count
            }).sum::<i32>() as i32
    }
}

// alternative solution
impl Solution {
    pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        grid.into_iter()
            .flat_map(|row| row.into_iter().rev().take_while(|v| v.is_negative()))
            .count() as _
    }
}