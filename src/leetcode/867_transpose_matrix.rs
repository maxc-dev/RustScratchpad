impl Solution {
    pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut transposed = vec![vec![0;matrix.len()];matrix[0].len()];
        for (r, row) in matrix.into_iter().enumerate() {
            for (c, val) in row.into_iter().enumerate() {
                transposed[c][r] = val;
            }
        }
        transposed
    }
}

// another solution, more idiomatic
impl Solution {
    pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        (0..matrix[0].len())
            .map(|col| matrix.iter().map(|row| row[col]).collect())
            .collect()
    }
}