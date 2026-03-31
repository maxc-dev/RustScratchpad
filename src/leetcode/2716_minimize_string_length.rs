pub fn minimized_string_length(s: String) -> i32 {
    s.chars().collect::<std::collections::HashSet<_>>().len() as i32
}