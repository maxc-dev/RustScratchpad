impl Solution {
    pub fn minimum_index(capacity: Vec<i32>, item_size: i32) -> i32 {
        let mut index = -1;
        let mut current_smallest_cap = 101;
        for (i, cap) in capacity.into_iter().enumerate() {
            if item_size == cap {
                return i as i32;
            }
            if item_size < cap && cap < current_smallest_cap {
                index = i as i32;
                current_smallest_cap = cap;
            }
        }

        index
    }
}