impl Solution {
    pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
        let len = mat.len();
        let mut left = 0usize;
        let mut right = len-1;
        let mut sum = 0;
        for r in 0..len {
            sum += mat[r][left];
            sum += mat[r][right];
            left += 1;
            right -= 1;
        }
        if len % 2 == 1 {
            let mid = (len - 1) / 2;
            sum -= mat[mid][mid];
        }
        sum
    }
}