pub fn decode(encoded: &[i32], first: i32) -> Vec<i32> {
    let decoded: Vec<i32> = encoded
        .iter()
        .scan(first, |state, &e| {
            *state ^= e;
            Some(*state)
        })
        .collect();

    let mut result = vec![first];
    result.extend(decoded);

    result
}
