impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let letters = vec![
            vec!['a', 'b', 'c'],        // index = 2
            vec!['d', 'e', 'f'],
            vec!['g', 'h', 'i'],
            vec!['j', 'k', 'l'],
            vec!['m', 'n', 'o'],
            vec!['p', 'q', 'r', 's'],
            vec!['t', 'u', 'v'],
            vec!['w', 'x', 'y', 'z'],   // index = 9
        ];

        fn solve(current: &mut Vec<char>, digits: &Vec<u32>, letters: &Vec<Vec<char>>, res: &mut Vec<String>) {
            if digits.len() == current.len() {
                res.push(current.clone().into_iter().collect::<String>());
                return;
            }

            let next_digit = digits[current.len()];
            for c in letters[(next_digit - 2) as usize].clone() {
                current.push(c);
                solve(current, digits, letters, res);
                current.pop();
            }
        }

        let mut res = Vec::new();
        let digits = digits.chars().map(|d| d.to_digit(10).unwrap()).collect::<Vec<_>>();
        solve(&mut Vec::new(), &digits, &letters, &mut res);
        res
    }
}