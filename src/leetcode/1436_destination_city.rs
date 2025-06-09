pub fn dest_city(paths: Vec<Vec<String>>) -> String {
    use std::collections::HashSet;
    let filter = paths.iter().map(|v| v[0].clone()).collect::<HashSet<_>>();
    paths.into_iter().map(|v| v[1].clone()).find(|s| !filter.contains(s)).unwrap()
}