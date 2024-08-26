pub fn main() {
    let answer = is_valid("()".to_string());
    println!("Answer: {:?}", answer);
}

pub fn is_valid(s: String) -> bool {
    let mut stack = Vec::new();
    for c in s.chars() {
        match c {
            '(' => stack.push(')'),
            '[' => stack.push(']'),
            '{' => stack.push('}'),
            ')' | ']' | '}' => if stack.pop() != Some(c) { return false },
            _ => {}
        }
    }
    stack.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_parentheses() {
        assert!(is_valid("()".to_string()));
        assert!(is_valid("()[]{}".to_string()));
        assert!(is_valid("{[]}".to_string()));
    }

    #[test]
    fn test_invalid_parentheses() {
        assert!(!is_valid("(]".to_string()));
        assert!(!is_valid("([)]".to_string()));
        assert!(!is_valid("{[}".to_string()));
    }

    #[test]
    fn test_mixed_types() {
        assert!(is_valid("{[()]}".to_string()));
        assert!(!is_valid("{[(])}".to_string()));
    }

    #[test]
    fn test_empty_string() {
        assert!(is_valid("".to_string()));
    }

    #[test]
    fn test_single_type() {
        assert!(!is_valid("(".to_string()));
        assert!(!is_valid(")".to_string()));
        assert!(!is_valid("[".to_string()));
        assert!(!is_valid("]".to_string()));
        assert!(!is_valid("{".to_string()));
        assert!(!is_valid("}".to_string()));
    }

    #[test]
    fn test_long_valid_parentheses() {
        assert!(is_valid("()[]{}()[]{}".to_string()));
        assert!(is_valid("(((())))".to_string()));
        assert!(is_valid("{{{{}}}}".to_string()));
    }

    #[test]
    fn test_long_invalid_parentheses() {
        assert!(!is_valid("()[]{}(]".to_string()));
        assert!(!is_valid("(((()))".to_string()));
        assert!(!is_valid("{{{{}}}".to_string()));
    }

    #[test]
    fn test_parentheses_with_spaces() {
        assert!(is_valid("() [] {}".to_string()));
        assert!(!is_valid("( ) [ ] { } ( ]".to_string()));
    }
}