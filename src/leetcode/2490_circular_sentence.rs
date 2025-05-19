pub fn is_circular_sentence(sentence: String) -> bool {
    if sentence.chars().next().unwrap() != sentence.chars().last().unwrap() {
        return false;
    }
    let s = sentence.split(" ").collect::<Vec<&str>>();

    for i in 0..s.len()-1 {
        if s[i].chars().last().unwrap() != s[i+1].chars().next().unwrap() {
            return false;
        }
    }

    true
}