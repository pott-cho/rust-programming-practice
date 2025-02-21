use std::iter::once;

pub fn decode(encoded: &[i32], first: i32) -> Vec<i32> {
    once(first)
        .chain(encoded.iter().scan(first, |state, &e| {
            *state ^= e;
            Some(*state)
        }))
        .collect()
}
