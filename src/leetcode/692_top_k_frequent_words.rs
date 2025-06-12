pub fn top_k_frequent(words: Vec<String>, k: i32) -> Vec<String> {
    use std::collections::{BinaryHeap, HashMap};
    let freqs = words.into_iter().fold(HashMap::new(), |mut acc, word| {
        *acc.entry(word).or_insert(0) += 1;
        acc
    });
    let mut heap = BinaryHeap::<(i32, String)>::with_capacity(k as usize + 1);
    for (word, freq) in freqs {
        heap.push((-freq, word));
        if heap.len() > k as usize {
            heap.pop();
        }
    }
    let mut vec = Vec::with_capacity(k as usize);
    while let Some((_, word)) = heap.pop() {
        vec.push(word);
    }
    vec.reverse();
    vec
}