pub fn main() {
    let answer = longest_common_prefix(vec!["flower".to_string(), "flow".to_string(), "flight".to_string()]);
    println!("Answer: {:?}", answer);
}

pub fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.is_empty() {
        return String::from("")
    }

    let mut prefix: String = strs[0].clone();
    for i in 1..strs.len() {
        let word = &strs[i];
        if prefix.len() > word.len() {
            prefix.split_off(word.len());
        }
        for (j, c) in word.chars().enumerate() {
            if prefix.len() <= j || !c.eq(&prefix.chars().nth(j).unwrap()) {
                prefix.split_off(j);
                break;
            }
            if prefix.is_empty() {
                return prefix
            }
        }
    }
    prefix
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_strings() {
        assert_eq!(longest_common_prefix(vec![]), "");
    }

    #[test]
    fn test_single_string() {
        assert_eq!(longest_common_prefix(vec!["single".to_string()]), "single");
    }

    #[test]
    fn test_common_prefix() {
        assert_eq!(longest_common_prefix(vec!["flower".to_string(), "flow".to_string(), "flight".to_string()]), "fl");
    }

    #[test]
    fn test_no_common_prefix() {
        assert_eq!(longest_common_prefix(vec!["dog".to_string(), "racecar".to_string(), "car".to_string()]), "");
    }

    #[test]
    fn test_varying_lengths() {
        assert_eq!(longest_common_prefix(vec!["interstellar".to_string(), "internet".to_string(), "internal".to_string()]), "inter");
    }

    #[test]
    fn test_special_characters() {
        assert_eq!(longest_common_prefix(vec!["@home".to_string(), "@homework".to_string(), "@homebase".to_string()]), "@home");
    }

    #[test]
    fn test_empty_strings() {
        assert_eq!(longest_common_prefix(vec!["".to_string(), "".to_string(), "".to_string()]), "");
    }

    #[test]
    fn test_mixed_empty_and_non_empty() {
        assert_eq!(longest_common_prefix(vec!["".to_string(), "nonempty".to_string()]), "");
    }

    #[test]
    fn test_all_same_strings() {
        assert_eq!(longest_common_prefix(vec!["same".to_string(), "same".to_string(), "same".to_string()]), "same");
    }

    #[test]
    fn test_case_sensitivity() {
        assert_eq!(longest_common_prefix(vec!["Case".to_string(), "case".to_string()]), "");
    }

    #[test]
    fn test_short_letters() {
        assert_eq!(longest_common_prefix(vec!["ab".to_string(), "a".to_string()]), "a");
    }
}