pub fn finding_users_active_minutes(logs: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
    use std::collections::{HashSet,HashMap};
    let map: HashMap<i32, i32> = logs.into_iter().fold(HashMap::new(), |mut acc, vec| {
        acc.entry(vec[0]).or_insert(HashSet::new()).insert(vec[1]);
        acc //user -> { mins, ..., mins }
    }).into_iter()
        .map(|(k, v)| v.len() as i32) // vec of count of minutes
        .fold(HashMap::new(), |mut acc, num| {
            *acc.entry(num).or_insert(0) += 1;
            acc
        }); // min -> freq

    let mut vec = vec![0; k as usize];
    for (key, v) in map {
        vec[(key - 1) as usize] = v;
    }

    vec
}
// todo it's possible to do this without the second into_iter()