pub fn num_jewels_in_stones(jewels: &str, stones: &str) -> i32 {
    let result = stones.chars().filter(|x| jewels.contains(*x)).count();

    result.try_into().unwrap()
}
