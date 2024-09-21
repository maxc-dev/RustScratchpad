pub fn main() {
    let accounts = vec![vec![1, 2, 3], vec![3, 2, 1]];
    println!("Richest customer wealth: {:?}", maximum_wealth(accounts));
}

pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
    accounts.iter().map(|acc| acc.iter().sum()).max().unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_case() {
        let accounts = vec![vec![1, 2, 3], vec![3, 2, 1]];
        assert_eq!(maximum_wealth(accounts), 6);
    }

    #[test]
    fn test_single_customer() {
        let accounts = vec![vec![10, 20, 30]];
        assert_eq!(maximum_wealth(accounts), 60);
    }

    #[test]
    fn test_multiple_customers() {
        let accounts = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        assert_eq!(maximum_wealth(accounts), 24);
    }

    #[test]
    fn test_empty_accounts() {
        let accounts: Vec<Vec<i32>> = vec![];
        assert_eq!(maximum_wealth(accounts), 0);
    }

    #[test]
    fn test_empty_customer_accounts() {
        let accounts = vec![vec![], vec![1, 2, 3]];
        assert_eq!(maximum_wealth(accounts), 6);
    }

    #[test]
    fn test_large_numbers() {
        let accounts = vec![vec![1_000_000, 2_000_000], vec![3_000_000, 4_000_000]];
        assert_eq!(maximum_wealth(accounts), 7_000_000);
    }
}