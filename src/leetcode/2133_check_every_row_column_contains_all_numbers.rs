use std::collections::HashSet;

impl Solution {
    pub fn check_valid(matrix: Vec<Vec<i32>>) -> bool {
        let count = matrix.len();
        for col in 0..count {
            if count != (0..count).map(|row| matrix[row][col]).collect::<HashSet<_>>().len() {
                return false;
            }
        }

        for row in matrix.into_iter() {
            if count != row.into_iter().collect::<HashSet<_>>().len() {
                return false;
            }
        }
        true
    }
}