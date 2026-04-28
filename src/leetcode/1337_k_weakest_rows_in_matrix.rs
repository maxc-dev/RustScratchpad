use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let mut heap = BinaryHeap::<Reverse<(i32, i32)>>::new();

        mat.into_iter().enumerate().for_each(|(index, row)| {
            heap.push(Reverse((row.into_iter().filter(|i| *i == 1).count() as i32, index as i32)));
        });

        let mut res = Vec::with_capacity(k as usize);

        for _ in 0..k {
            if let Some(Reverse((_, idx))) = heap.pop() {
                res.push(idx);
            }
        }

        res
    }
}