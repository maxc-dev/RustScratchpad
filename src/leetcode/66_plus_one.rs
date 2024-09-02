pub fn main() {
    let answer = plus_one(vec![1, 2, 3]);
    println!("Answer: {:?}", answer);
}

pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut digits = digits;
    for i in digits.iter_mut().rev() {
        if *i == 9 {
            *i = 0;
        } else {
            *i += 1;
            return digits;
        }
    }

    digits.insert(0, 1);
    digits
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_digit_less_than_9() {
        assert_eq!(plus_one(vec![5]), vec![6]);
    }

    #[test]
    fn test_single_digit_equal_to_9() {
        assert_eq!(plus_one(vec![9]), vec![1, 0]);
    }

    #[test]
    fn test_multiple_digits_no_carry() {
        assert_eq!(plus_one(vec![1, 2, 3]), vec![1, 2, 4]);
    }

    #[test]
    fn test_multiple_digits_with_carry() {
        assert_eq!(plus_one(vec![1, 2, 9]), vec![1, 3, 0]);
    }

    #[test]
    fn test_all_digits_are_9() {
        assert_eq!(plus_one(vec![9, 9, 9]), vec![1, 0, 0, 0]);
    }

    #[test]
    fn test_leading_zeros() {
        assert_eq!(plus_one(vec![0, 0, 1]), vec![0, 0, 2]);
    }

    #[test]
    fn test_large_number_of_digits() {
        let large_vec = vec![1; 1000];
        let mut expected_vec = vec![1; 999];
        expected_vec.push(2);
        assert_eq!(plus_one(large_vec), expected_vec);
    }

    #[test]
    fn test_mixed_positive_and_negative_numbers() {
        assert_eq!(plus_one(vec![-1, 2, 3]), vec![-1, 2, 4]);
    }
}