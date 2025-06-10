pub fn sort_people(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
    let mut people = names.into_iter()
        .zip(heights.into_iter())
        .collect::<Vec<_>>();
    people.sort_by(|a, b| b.1.cmp(&a.1));
    people.into_iter().map(|(name, _)| name).collect()
}