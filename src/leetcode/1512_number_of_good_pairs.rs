use std::collections::HashMap;

pub fn main() {
    let nums = vec![1, 2, 3, 1, 1, 3];
    println!("Number of good pairs: {:?}", num_identical_pairs(nums));
}

pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
    let mut map = HashMap::new();
    let mut count = 0;
    for i in nums {
        let k = map.entry(i).or_insert(0);
        count += *k;
        *k += 1;
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_case() {
        let nums = vec![1, 2, 3, 1, 1, 3];
        assert_eq!(num_identical_pairs(nums), 4);
    }

    #[test]
    fn test_no_pairs() {
        let nums = vec![1, 2, 3, 4, 5];
        assert_eq!(num_identical_pairs(nums), 0);
    }

    #[test]
    fn test_all_identical() {
        let nums = vec![1, 1, 1, 1];
        assert_eq!(num_identical_pairs(nums), 6);
    }

    #[test]
    fn test_single_element() {
        let nums = vec![1];
        assert_eq!(num_identical_pairs(nums), 0);
    }

    #[test]
    fn test_empty_list() {
        let nums: Vec<i32> = vec![];
        assert_eq!(num_identical_pairs(nums), 0);
    }

    #[test]
    fn test_large_numbers() {
        let nums = vec![100000, 100000, 100000];
        assert_eq!(num_identical_pairs(nums), 3);
    }

    #[test]
    fn test_mixed_numbers() {
        let nums = vec![1, 2, 2, 3, 3, 3];
        assert_eq!(num_identical_pairs(nums), 4);
    }

    #[test]
    fn test_negative_numbers() {
        let nums = vec![-1, -1, -2, -2, -2];
        assert_eq!(num_identical_pairs(nums), 4);
    }

    #[test]
    fn test_large_input() {
        let nums = vec![1; 1000];
        assert_eq!(num_identical_pairs(nums), 499500);
    }
}