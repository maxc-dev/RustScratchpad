pub fn split_words_by_separator(words: Vec<String>, separator: char) -> Vec<String> {
    words.into_iter()
        .flat_map(|v| v.split(separator).map(|c| c.to_string()).filter(|c| !c.is_empty()).collect::<Vec<_>>())
        .collect::<Vec<String>>()
}