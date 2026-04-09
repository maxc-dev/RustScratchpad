impl Solution {
    pub fn find_numbers(nums: Vec<i32>) -> i32 {
        let digit_len = |mut num: i32| {
            let mut count = 0;
            while num > 0 {
                count += 1;
                num = num / 10;
            }
            count
        };

        nums.into_iter().filter(|i| digit_len(*i) % 2 == 0).count() as i32
    }
}