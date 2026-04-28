impl Solution {
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        let mut count = 0;
        let Y: usize = grid.len();
        let X: usize = grid[0].len();

        let is_water = |y: i32, x: i32, grid: &Vec<Vec<i32>>| -> bool {
            if y == -1 || y == Y as i32 || x == -1 || x == X as i32 {
                true
            } else if grid[y as usize][x as usize] == 0 {
                true
            } else {
                false
            }
        };

        for y in 0..Y {
            for x in 0..X {
                if grid[y][x] == 1 {
                    if is_water(y as i32, x as i32 -1, &grid) { count += 1 }
                    if is_water(y as i32 -1, x as i32, &grid) { count += 1 }
                    if is_water(y as i32 +1, x as i32, &grid) { count += 1 }
                    if is_water(y as i32, x as i32 +1, &grid) { count += 1 }
                }
            }
        }

        count
    }
}