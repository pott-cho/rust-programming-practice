pub fn num_jewels_in_stones(jewels: &str, stones: &str) -> i32 {
    let mut result = 0;
    for c in stones.chars() {
        if jewels.contains(c) {
            result += 1;
        }
    }

    result
}
