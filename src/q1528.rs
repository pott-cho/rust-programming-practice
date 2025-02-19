pub fn restore_string(s: &str, indices: &[usize]) -> String {
    let ch: Vec<char> = s.chars().collect();
    let mut combi: Vec<(char, &usize)> = ch.into_iter().zip(indices).collect();
    combi.sort_by_key(|&(_, index)| index);
    combi.iter().map(|(x, _)| x).collect()
}
