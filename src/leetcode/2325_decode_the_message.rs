pub fn decode_message(key: String, message: String) -> String {
    use std::collections::HashMap;
    let mut map = HashMap::with_capacity(26);
    let mut next = b'a';

    for c in key.chars().filter(|c| *c != ' ') {
        map.entry(c).or_insert_with(|| {
            let res = next;
            next += 1;
            res
        });
        if next > b'z' {
            break;
        }
    }

    message.chars().into_iter().map(|c| if c != ' ' {
        *map.get(&c).unwrap() as char
    } else {
        ' '
    }).collect()
}