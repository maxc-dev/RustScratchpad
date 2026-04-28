impl Solution {
    pub fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
        let mut image = image;
        let fill = image[sr as usize][sc as usize];

        if fill == color {
            return image;
        }

        fn fill_curr(image: &mut Vec<Vec<i32>>, fill: i32, sr: i32, sc: i32, color: i32) {
            if sr == -1 || sr == image.len() as i32 || sc == -1 || sc == image[0].len() as i32 {
                return;
            }
            if image[sr as usize][sc as usize] == fill {
                image[sr as usize][sc as usize] = color;
                fill_curr(image, fill, sr + 1, sc, color);
                fill_curr(image, fill, sr, sc + 1, color);
                fill_curr(image, fill, sr - 1, sc, color);
                fill_curr(image, fill, sr, sc - 1, color);
            }
        }

        fill_curr(&mut image, fill, sr, sc, color);
        image
    }
}