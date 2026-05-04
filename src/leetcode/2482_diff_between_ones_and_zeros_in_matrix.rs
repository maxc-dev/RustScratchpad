impl Solution {
    pub fn ones_minus_zeros(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let row_len = grid.len();
        let col_len = grid[0].len();
        let mut one_row = vec![0; row_len];
        let mut zero_row = vec![0; row_len];
        let mut one_col = vec![0; col_len];
        let mut zero_col = vec![0; col_len];

        for (r_idx, row) in grid.iter().enumerate() {
            for (c_idx, val) in row.iter().enumerate() {
                if *val == 0 {
                    zero_col[c_idx] += 1;
                    zero_row[r_idx] += 1;
                } else {
                    one_col[c_idx] += 1;
                    one_row[r_idx] += 1;
                }
            }
        }

        let mut diff = grid;
        for r in 0..row_len {
            for c in 0..col_len {
                diff[r][c] = one_row[r] + one_col[c] - zero_row[r] - zero_col[c];
            }
        }
        diff
    }
}