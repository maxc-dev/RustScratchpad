pub fn percentage_letter(s: String, letter: char) -> i32 {
    let n = s.len() as f64;
    let count = s.chars().into_iter().filter(|c| letter == *c).count() as f64;
    ((count/n) * 100.0).floor() as i32
}