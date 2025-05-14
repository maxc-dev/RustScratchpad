pub fn replace_digits(s: String) -> String {
    if s.len() <= 1 {
        return s;
    }
    let mut buffer: Vec<u8> = s.chars().into_iter().map(|c| c as u8).collect::<Vec<u8>>();
    let z = '0' as u8;

    for i in (1..buffer.len()).step_by(2) {
        buffer[i] += buffer[i-1] - z;
    }

    buffer.into_iter().map(|i| i as char).collect()
}