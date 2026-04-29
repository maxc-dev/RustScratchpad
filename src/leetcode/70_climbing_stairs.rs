impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        fn climb(n: i32, cache: &mut Vec<i32>) -> i32 {
            if n <= 3 {
                return n;
            }
            if cache[n as usize] == 0 {
                let ans = climb(n-1, cache) + climb(n-2, cache);
                cache[n as usize] = ans;
                ans
            } else {
                cache[n as usize]
            }
        }
        climb(n, &mut vec![0;50])
    }
}