pub fn decode(encoded: &[i32], first: i32) -> Vec<i32> {
    let mut result = Vec::new();
    result.push(first);

    let mut temp = first;
    for e in encoded {
        temp ^= e;
        result.push(temp);
    }

    result
}
