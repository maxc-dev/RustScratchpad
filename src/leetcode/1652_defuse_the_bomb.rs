pub fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32> {
    if k == 0 {
        return code.iter().map(|c| 0).collect();
    }
    let n = code.len();
    let mut vec: Vec<i32> = Vec::with_capacity(n);
    for i in 0..n {
        let mut local = 0;
        if k > 0 {
            for j in i+1..i+1+(k as usize) {
                local += code[j % n];
            }
        } else {
            let i = i as i32;
            for j in (i+k..i).rev() {
                local += code[(j.rem_euclid(n as i32)) as usize];
            }
        }
        vec.push(local);
    }

    vec
}