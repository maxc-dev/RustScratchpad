use std::collections::HashMap;

fn main() {
    let nums = vec![3, 2, 3];
    println!("Majority element: {:?}", majority_element(nums));
}

pub fn majority_element(nums: Vec<i32>) -> i32 {
    let mut map = HashMap::new();
    let mut max_index = 0;
    let mut max_value = 0;
    let n = nums.len()/2;

    for i in nums.iter() {
        let count = map.entry(i).or_insert(0);
        *count+=1;
        if *count > max_value {
            max_value = *count;
            max_index = *i;
        }
        if max_value > n {
            return max_index;
        }
    }

    max_index
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clear_majority_element() {
        let nums = vec![3, 2, 3];
        assert_eq!(majority_element(nums), 3);
    }

    #[test]
    fn test_all_elements_the_same() {
        let nums = vec![2, 2, 2, 2];
        assert_eq!(majority_element(nums), 2);
    }

    #[test]
    fn test_majority_element_in_the_middle() {
        let nums = vec![1, 2, 3, 2, 2];
        assert_eq!(majority_element(nums), 2);
    }

    #[test]
    fn test_negative_numbers() {
        let nums = vec![-1, -1, -1, 2, 2];
        assert_eq!(majority_element(nums), -1);
    }

    #[test]
    fn test_large_input() {
        let nums = vec![1; 1000];
        assert_eq!(majority_element(nums), 1);
    }
}
