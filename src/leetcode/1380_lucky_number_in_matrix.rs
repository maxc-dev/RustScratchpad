impl Solution {
    pub fn lucky_numbers(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let smallest_row: Vec<i32> = matrix.iter().map(|row| row.iter().min().copied().unwrap()).collect();
        let N = matrix[0].len();
        let largest_col: Vec<i32> = (0..N).map(|c| (0..matrix.len()).map(|r| matrix[r][c]).max().unwrap()).collect();

        let mut res = Vec::new();
        for (r, row) in matrix.iter().enumerate() {
            for (c, &num) in row.iter().enumerate() {
                if smallest_row[r] == num && largest_col[c] == num {
                    res.push(num);
                }
            }
        }
        res
    }
}