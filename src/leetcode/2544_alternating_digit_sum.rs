pub fn alternate_digit_sum(n: i32) -> i32 {
    let s = n.to_string();
    let mut sum = 0;
    let mut mul = 1;
    for c in s.chars() {
        sum += (c as u8 - b'0') as i8 * mul;
        mul *= -1;
    }
    sum as i32
}