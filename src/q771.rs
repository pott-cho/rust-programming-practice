pub fn num_jewels_in_stones(jewels: &str, stones: &str) -> Result<i32, String> {
    let result = stones.chars().filter(|x| jewels.contains(*x)).count();

    result
        .try_into()
        .map_err(|_| String::from("stones size exceeds i32 range."))
}
