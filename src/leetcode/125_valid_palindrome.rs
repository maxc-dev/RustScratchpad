pub fn main() {

}

// this is my attempt at solving without using a filter
// not nearly as idiomatic as the other solution
pub fn is_palindrome_old(s: String) -> bool {
    if s.is_empty() {
        return true;
    }
    let mut last = s.len()-1;
    let s: Vec<char> = s.chars().collect();
    let mut first: usize = 0;

    while last > first {
        while !s[first].is_alphanumeric() {
            first += 1;
        }
        while !s[last].is_alphanumeric() {
            last -= 1;
        }
        if !s[first].eq_ignore_ascii_case(&s[last]) {
            return false;
        }
        first += 1;
        last -= 1;
    }

    true
}


pub fn is_palindrome(s: String) -> bool {
    let s: Vec<char> = s.chars().filter(|c| c.is_alphanumeric()).collect();

    s.iter()
        .zip(s.iter().rev())
        .take(&s.len() / 2)
        .all(|(a, b)| a.eq_ignore_ascii_case(b))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_string() {
        assert_eq!(is_palindrome("".to_string()), true);
    }

    #[test]
    fn test_short_strings_whitespace() {
        assert_eq!(is_palindrome(" aba ".to_string()), true);
        assert_eq!(is_palindrome(" aa ".to_string()), true);
        assert_eq!(is_palindrome(" a a ".to_string()), true);
        assert_eq!(is_palindrome(" a  a ".to_string()), true);
        assert_eq!(is_palindrome(" a ".to_string()), true);
    }

    #[test]
    fn test_single_character() {
        assert_eq!(is_palindrome("a".to_string()), true);
    }

    #[test]
    fn test_simple_palindrome() {
        assert_eq!(is_palindrome("racecar".to_string()), true);
    }

    #[test]
    fn test_non_palindrome() {
        assert_eq!(is_palindrome("hello".to_string()), false);
    }

    #[test]
    fn test_palindrome_with_spaces_and_special_characters() {
        assert_eq!(is_palindrome("A man, a plan, a canal: Panama".to_string()), true);
    }

    #[test]
    fn test_mixed_case_palindrome() {
        assert_eq!(is_palindrome("No lemon, no melon".to_string()), true);
    }
}