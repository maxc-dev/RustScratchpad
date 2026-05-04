use std::collections::HashMap;

impl Solution {
    pub fn first_complete_index(arr: Vec<i32>, mat: Vec<Vec<i32>>) -> i32 {
        let r_len = mat.len();
        let c_len = mat[0].len();
        let mut row = vec![0usize; r_len];
        let mut col = vec![0usize; c_len];

        let mat_idx = mat.into_iter().enumerate().fold(
            HashMap::with_capacity(r_len * c_len),
            |mut acc, (row_idx, row)| {
                for (col_idx, val) in row.into_iter().enumerate() {
                    *acc.entry(val).or_insert((row_idx, col_idx));
                }
                acc
            },
        ); // arr val -> (r,c)

        for (idx, num) in arr.iter().enumerate() {
            let (r, c) = mat_idx.get(&num).unwrap();
            row[*r] += 1;
            col[*c] += 1;
            if row[*r] == c_len || col[*c] == r_len {
                return idx as i32;
            }
        }
        (r_len * c_len) as i32 - 1
    }
}
