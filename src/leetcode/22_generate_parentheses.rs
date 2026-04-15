impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        fn solve(open: i32, close: i32, n: usize, current: &mut Vec<char>, res: &mut Vec<String>) {
            if current.len() == (n * 2) {
                res.push(current.clone().into_iter().collect::<String>());
                return;
            }
            if open > 0 {
                current.push('(');
                solve(open - 1, close, n, current, res);
                current.pop();
            }
            if close > open {
                current.push(')');
                solve(open, close - 1, n, current, res);
                current.pop();
            }
        }

        let mut res = Vec::new();
        solve(n, n, n as usize, &mut Vec::with_capacity(n as usize * 2), &mut res);
        res
    }
}
