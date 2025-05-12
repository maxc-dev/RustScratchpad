pub fn main() {
    let answer = max_profit(vec![7, 1, 5, 3, 6, 4]);
    println!("Answer: {:?}", answer);
}

pub fn max_profit(prices: Vec<i32>) -> i32 {
    use std::cmp::min;
    let mut profit = 0;
    let mut low = i32::MAX;

    for i in prices {
        if low < i {
            profit += i - low;
            low = i;
        } else {
            low = min(low, i);
        }
    }

    profit
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        assert_eq!(max_profit(vec![]), 0);
    }

    #[test]
    fn test_single_price() {
        assert_eq!(max_profit(vec![5]), 0);
    }

    #[test]
    fn test_no_profit() {
        assert_eq!(max_profit(vec![9, 7, 4, 3, 1]), 0);
    }

    #[test]
    fn test_increasing_prices() {
        assert_eq!(max_profit(vec![1, 2, 3, 4, 5]), 4);
    }

    #[test]
    fn test_fluctuating_prices() {
        assert_eq!(max_profit(vec![7, 1, 5, 3, 6, 4]), 7);
    }

    #[test]
    fn test_flat_prices() {
        assert_eq!(max_profit(vec![3, 3, 3, 3]), 0);
    }

    #[test]
    fn test_zigzag_prices() {
        assert_eq!(max_profit(vec![1, 3, 2, 4, 3, 5]), 6);
    }
}
