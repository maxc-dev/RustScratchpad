use std::collections::HashMap;

pub fn main() {
    let answer = two_sum(vec![1, 2, 3, 4, 5, 6, 7, 8], 5);
    println!("Answer: {:?}", answer);
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::new();
    for (i, num) in nums.iter().enumerate() {
        match map.get(&(target - *num)) {
            Some(&i2) => return vec![i2, i as i32],
            None => map.insert(*num, i as i32)
        };
    }
    vec![]
}

#[cfg(test)]
mod two_sum_tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        assert_eq!(two_sum(vec![1, 2, 3, 4, 5, 6, 7, 8], 5), vec![1, 2]);
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(two_sum(vec![3, 3], 6), vec![0, 1]);
    }

    #[test]
    fn test_two_sum_no_solution() {
        assert_eq!(two_sum(vec![1, 2, 3], 7), vec![]);
        assert_eq!(two_sum(vec![1, 2, 3], 0), vec![]);
        assert_eq!(two_sum(vec![], 1), vec![]);
        assert_eq!(two_sum(vec![], 0), vec![]);
    }

    #[test]
    fn test_two_sum_with_large_numbers() {
        assert_eq!(two_sum(vec![10000000, 20000000, 30000000], 50000000), vec![1, 2]);
        assert_eq!(two_sum(vec![10000000, 20000000, 30000000], 30000000), vec![0, 1]);
    }

    #[test]
    fn test_two_sum_with_repeated_numbers() {
        assert_eq!(two_sum(vec![1, 1, 1, 1, 1], 2), vec![0, 1]);
        assert_eq!(two_sum(vec![2, 2, 2, 2, 2], 4), vec![0, 1]);
    }
}