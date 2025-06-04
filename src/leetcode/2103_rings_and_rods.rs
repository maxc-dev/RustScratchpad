pub fn count_points(rings: String) -> i32 {
    use std::collections::{HashMap,HashSet};
    let rings = rings.chars().collect::<Vec<_>>();
    let mut map = HashMap::with_capacity(10);
    for i in (1..rings.len()).step_by(2) {
        map.entry(rings[i as usize])
            .or_insert(HashSet::with_capacity(3))
            .insert(rings[(i-1) as usize]);
    }
    map.values().filter(|s| s.len() == 3).count() as i32
}