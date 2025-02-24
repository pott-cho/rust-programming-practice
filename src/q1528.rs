pub fn restore_string(s: &str, indices: &[usize]) -> String {
    let mut combi: Vec<(char, &usize)> = s.chars().zip(indices.iter()).collect();
    combi.sort_unstable_by(|a, b| a.1.cmp(b.1));
    combi.iter().map(|(x, _)| x).collect()
}
