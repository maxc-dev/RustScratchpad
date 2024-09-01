fn main() {
    let answer = length_of_last_word("Hello World".to_string());
    println!("Answer: {:?}", answer);
}

pub fn length_of_last_word(s: String) -> i32 {
    let mut check = false;
    let mut count = 0;
    for c in s.chars().rev() {
        match c {
            ' ' => { if check { return count; } }
            _ => { check = true; count += 1; }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_string() {
        assert_eq!(length_of_last_word("".to_string()), 0);
    }

    #[test]
    fn test_only_spaces() {
        assert_eq!(length_of_last_word("     ".to_string()), 0);
    }

    #[test]
    fn test_one_word() {
        assert_eq!(length_of_last_word("hello".to_string()), 5);
    }

    #[test]
    fn test_multiple_words() {
        assert_eq!(length_of_last_word("hello world".to_string()), 5);
    }

    #[test]
    fn test_trailing_spaces() {
        assert_eq!(length_of_last_word("hello world   ".to_string()), 5);
    }

    #[test]
    fn test_leading_spaces() {
        assert_eq!(length_of_last_word("   hello world".to_string()), 5);
    }

    #[test]
    fn test_multiple_spaces_between_words() {
        assert_eq!(length_of_last_word("hello   world".to_string()), 5);
    }

    #[test]
    fn test_special_characters() {
        assert_eq!(length_of_last_word("hello world!".to_string()), 6);
    }

    #[test]
    fn test_numbers() {
        assert_eq!(length_of_last_word("hello 12345".to_string()), 5);
    }

    #[test]
    fn test_mixed_alphanumeric() {
        assert_eq!(length_of_last_word("hello world123".to_string()), 8);
    }

    #[test]
    fn test_unicode_characters() {
        assert_eq!(length_of_last_word("hello 世界".to_string()), 2);
    }
}