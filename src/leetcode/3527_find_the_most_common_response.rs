pub fn find_common_response(responses: Vec<Vec<String>>) -> String {
    use std::collections::{HashMap, HashSet};
    let mut map: HashMap<String, i32> = HashMap::new();

    for daily_responses in responses {
        for response in daily_responses.into_iter().collect::<HashSet<String>>() {
            map.entry(response.clone()).and_modify(|v| *v += 1).or_insert(1);
        }
    }

    map.into_iter()
        .max_by(|(k1, v1), (k2, v2)| v1.cmp(v2).then_with(|| k2.cmp(k1)))
        .map(|(k, _)| k)
        .unwrap_or_default()
}