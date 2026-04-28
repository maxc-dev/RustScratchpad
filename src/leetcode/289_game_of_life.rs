impl Solution {
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        let board_ref = board.clone();
        let mut res = board.clone();

        let is_live = |board: &Vec<Vec<i32>>, r: i32, c: i32| -> bool {
            if r < 0 || r >= board.len() as i32 || c < 0 || c >= board[0].len() as i32 {
                false
            } else {
                board[r as usize][c as usize] == 1
            }
        };

        for (i, row) in res.into_iter().enumerate() {
            for (j, col) in row.clone().into_iter().enumerate() {
                let n = vec![(i-1, j-1),(i, j-1),(i+1, j-1),(i-1, j),(i+1, j),(i+1, j+1),(i, j+1),(i-1, j+1)]
                    .into_iter()
                    .filter(|(mi, mj)| is_live(&board_ref, *mi as i32, *mj as i32))
                    .count();
                let result = if col == 1 {
                    if n == 2 || n == 3 {
                        1
                    } else {
                        0
                    }
                } else {
                    if n == 3 {
                        1
                    } else {
                        0
                    }
                };
                board[i][j] = result;
            }
        }
    }
}