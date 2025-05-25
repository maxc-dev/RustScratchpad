pub fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
    use std::collections::HashMap;
    use std::cmp::min;
    let mut map = HashMap::new();
    let mut res: Vec<String> = Vec::new();
    let mut sum = usize::MAX;
    for (i, word) in list1.into_iter().enumerate() {
        map.entry(word).or_insert(i);
    }
    for (i, word) in list2.into_iter().enumerate() {
        if let Some(idx) = map.get(&word) {
            let total = idx + i;
            if total < sum {
                res.clear();
                res.push(word);
                sum = total;
            } else if total == sum {
                res.push(word);
            }
        }
    }

    res
}