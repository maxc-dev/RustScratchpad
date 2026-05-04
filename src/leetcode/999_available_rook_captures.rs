impl Solution {
    pub fn num_rook_captures(board: Vec<Vec<char>>) -> i32 {
        let mut count = 0;
        let mut rook_row = 0;
        let mut rook_col = 0;

        'rook: for (r_idx, rows) in board.iter().enumerate() {
            for (c_idx, square) in rows.iter().enumerate() {
                if *square == 'R' {
                    rook_row = r_idx;
                    rook_col = c_idx;
                    break 'rook;
                }
            }
        }

        fn pawn_found<I: Iterator<Item = char>>(iter: I) -> i32 {
            for cell in iter {
                if cell == 'p' {
                    return 1;
                } else if cell == 'B' {
                    return 0;
                }
            }
            0
        }

        count += pawn_found((0..rook_col).rev().map(|c| board[rook_row][c]));
        count += pawn_found((rook_col+1..8).map(|c| board[rook_row][c]));
        count += pawn_found((0..rook_row).rev().map(|r| board[r][rook_col]));
        count += pawn_found((rook_row+1..8).map(|r| board[r][rook_col]));
        count
    }
}