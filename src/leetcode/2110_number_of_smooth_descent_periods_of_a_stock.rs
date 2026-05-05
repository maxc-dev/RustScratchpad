impl Solution {
    pub fn get_descent_periods(prices: Vec<i32>) -> i64 {
        let mut count = 0;
        let mut current = 0;
        for i in 1..prices.len() {
            if prices[i]+1 == prices[i-1] {
                current += 1;
                count += current;
            } else {
                current = 0;
            }
        }
        count + prices.len() as i64
    }
}