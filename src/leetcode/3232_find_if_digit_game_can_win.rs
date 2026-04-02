pub fn can_alice_win(nums: Vec<i32>) -> bool {
    let mut single = 0;
    let mut double = 0;
    for i in nums {
        if i < 10 {
            single += i;
        } else {
            double += i;
        }
    }
    single != double
}

pub fn can_alice_win(nums: Vec<i32>) -> bool {
    let sum = nums.iter().fold(0, |acc, num| if *num < 10 { acc + num } else { acc - num });
    sum != 0
}