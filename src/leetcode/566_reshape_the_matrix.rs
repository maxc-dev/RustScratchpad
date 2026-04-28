impl Solution {
    pub fn matrix_reshape(mat: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
        let R = mat.len();
        let C = mat[0].len();
        if R as i32 * C as i32 != r * c {
            return mat;
        } else if R as i32 == r && C as i32 == c {
            return mat;
        }
        let mut res: Vec<Vec<i32>> = Vec::with_capacity(r as usize);
        let mut current: Vec<i32> = Vec::with_capacity(c as usize);

        for i in 0..R {
            for j in 0..C {
                current.push(mat[i][j]);
                if current.len() as i32 == c {
                    res.push(current.clone());
                    current.clear();
                }
            }
        }

        res
    }
}