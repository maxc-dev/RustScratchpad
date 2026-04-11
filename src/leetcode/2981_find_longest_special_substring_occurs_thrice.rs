use std::collections::HashMap;

impl Solution {
    pub fn maximum_length(s: String) -> i32 {
        let mut map: HashMap<(char, usize), usize> = HashMap::new();

        for (i, c1) in s.chars().enumerate() {
            for (j, c2) in s.chars().enumerate().skip(i) {
                if c1 == c2 {
                    *map.entry((c1, j - i + 1)).or_insert(0) += 1;
                } else {
                    break;
                }
            }
        }

        map.into_iter()
            .filter_map(|((_, len), count)| if count >= 3 { Some(len as i32)} else { None })
            .max()
            .unwrap_or(-1)
    }
}