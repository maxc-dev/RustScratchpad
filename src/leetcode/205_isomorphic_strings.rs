pub fn is_isomorphic(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }
    use std::collections::{HashMap,HashSet};
    let mut map: HashMap<char, char> = HashMap::with_capacity(s.len());

    for i in 0..s.len() {
        let si = s.chars().nth(i).unwrap();
        let ti = t.chars().nth(i).unwrap();
        match map.get(&si) {
            Some(c) => if *c != ti { return false; },
            None => { map.insert(si, ti); }
        }
    }

    if map.len() != t.chars().collect::<HashSet<char>>().len() {
        return false;
    }

    true
}