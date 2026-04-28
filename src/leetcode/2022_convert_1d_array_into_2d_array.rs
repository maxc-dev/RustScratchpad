impl Solution {
    pub fn construct2_d_array(arr: Vec<i32>, r: i32, c: i32) -> Vec<Vec<i32>> {
        let N = arr.len();
        if N as i32 != r * c {
            return vec![];
        }
        let mut res: Vec<Vec<i32>> = Vec::with_capacity(r as usize);
        let mut current: Vec<i32> = Vec::with_capacity(c as usize);

        for num in arr.into_iter() {
            current.push(num);
            if current.len() as i32 == c {
                res.push(current.clone());
                current.clear();
            }
        }

        res
    }
}