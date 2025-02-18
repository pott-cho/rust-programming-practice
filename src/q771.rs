pub fn num_jewels_in_stones(jewels: &str, stones: &str) -> i32 {
    let result = stones.chars().filter(|x| jewels.contains(*x)).count();

    // stones 의 길이가 매우 커서 result가 i32 범위를 넘어가면 -1을 반환한다.
    result.try_into().unwrap_or(-1)
}
