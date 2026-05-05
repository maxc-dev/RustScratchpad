impl Solution {
    pub fn check_x_matrix(grid: Vec<Vec<i32>>) -> bool {
        let len = grid.len();
        let mut left = 0usize;
        let mut right = len-1;
        for r in 0..len {
            for c in 0..len {
                if c == left || c == right {
                    if grid[r][left] == 0 || grid[r][right] == 0 {
                        return false;
                    }
                } else if grid[r][c] != 0 {
                    return false;
                }
            }

            left += 1;
            right -= 1;
        }
        true
    }
}