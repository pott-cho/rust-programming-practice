pub fn decode(encoded: &[i32], first: i32) -> Vec<i32> {
    let mut result = vec![first];
    result.extend(encoded.iter().scan(first, |state, &e| {
        *state ^= e;
        Some(*state)
    }));

    result
}
