pub fn main() {
    let answer = num_jewels_in_stones("aA".to_string(), "aAAbbbb".to_string());
    println!("Answer: {:?}", answer);
}


pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
    use std::collections::HashSet;
    let set: HashSet<char> = jewels.chars().collect();

    stones.chars().filter(|c| set.contains(c)).count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_case() {
        assert_eq!(num_jewels_in_stones("aA".to_string(), "aAAbbbb".to_string()), 3);
    }

    #[test]
    fn test_no_jewels() {
        assert_eq!(num_jewels_in_stones("z".to_string(), "ZZZZ".to_string()), 0);
    }

    #[test]
    fn test_all_jewels() {
        assert_eq!(num_jewels_in_stones("abc".to_string(), "abcabc".to_string()), 6);
    }

    #[test]
    fn test_empty_stones() {
        assert_eq!(num_jewels_in_stones("aA".to_string(), "".to_string()), 0);
    }

    #[test]
    fn test_empty_jewels() {
        assert_eq!(num_jewels_in_stones("".to_string(), "aAAbbbb".to_string()), 0);
    }

    #[test]
    fn test_large_input() {
        let jewels = "a".repeat(1000);
        let stones = "a".repeat(1000);
        assert_eq!(num_jewels_in_stones(jewels, stones), 1000);
    }
}