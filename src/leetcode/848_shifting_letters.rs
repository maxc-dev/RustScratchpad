pub fn shifting_letters(s: String, shifts: Vec<i32>) -> String {
    let a = 'a' as u8;
    let mut count: u128 = 0;
    let mut chars: Vec<u8> = s.chars().map(|c| (c as u8) - a).collect();

    for i in (0..shifts.len()).rev() {
        count += (shifts[i] as u128);
        chars[i] = (chars[i] as u128 + count).rem_euclid(26) as u8;
    }

    chars.into_iter().map(|c| (c + a) as char).collect::<String>()
}