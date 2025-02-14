pub fn restore_string(s: &str, indices: &[usize]) -> String {
    let ch: Vec<char> = s.chars().collect();
    let n = indices.len();
    let mut result = [' '; 101];
    for i in 0..n {
        let index = indices[i];
        result[index] = ch[i];
    }

    result[0..n].iter().collect()
}
