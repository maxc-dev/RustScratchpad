pub fn main() {
    let answer = max_profit(vec![7, 1, 5, 3, 6, 4]);
    println!("Answer: {:?}", answer);
}

pub fn max_profit(prices: Vec<i32>) -> i32 {
    use std::cmp::{min, max};
    let mut low = i32::MAX;
    let mut profit = 0;

    for i in prices {
        profit = max(profit, i - low);
        low = min(i, low);
    }

    profit
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_prices() {
        assert_eq!(max_profit(vec![]), 0);
    }

    #[test]
    fn test_single_day_price() {
        assert_eq!(max_profit(vec![5]), 0);
    }

    #[test]
    fn test_increasing_prices() {
        assert_eq!(max_profit(vec![1, 2, 3, 4, 5]), 4);
    }

    #[test]
    fn test_decreasing_prices() {
        assert_eq!(max_profit(vec![5, 4, 3, 2, 1]), 0);
    }

    #[test]
    fn test_random_prices() {
        assert_eq!(max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
        assert_eq!(max_profit(vec![7, 6, 4, 3, 1]), 0);
        assert_eq!(max_profit(vec![2, 4, 1]), 2);
    }
}