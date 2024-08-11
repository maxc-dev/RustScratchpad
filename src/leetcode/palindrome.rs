pub fn main() {
    let answer = is_palindrome(121);
    println!("Answer: {:?}", answer);
}

fn is_palindrome(num: i32) -> bool {
    let word = num.to_string();
    let len = word.len();
    for (i, c) in word.chars().enumerate() {
        if c != word.chars().nth(len - 1 - i).unwrap() {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_palindrome_numbers() {
        assert!(is_palindrome(121));
        assert!(is_palindrome(1221));
        assert!(is_palindrome(12321));
    }

    #[test]
    fn test_non_palindrome_numbers() {
        assert!(!is_palindrome(123));
        assert!(!is_palindrome(1234));
        assert!(!is_palindrome(12345));
    }

    #[test]
    fn test_single_digit_numbers() {
        assert!(is_palindrome(0));
        assert!(is_palindrome(1));
        assert!(is_palindrome(9));
    }

    #[test]
    fn test_negative_numbers() {
        assert!(!is_palindrome(-121));
        assert!(!is_palindrome(-1221));
    }

    #[test]
    fn test_large_numbers() {
        assert!(is_palindrome(123454321));
        assert!(!is_palindrome(123456789));
    }
}