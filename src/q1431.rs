pub fn kids_with_candies(candies: &[i32], extra_candies: i32) -> Vec<bool> {
    // 1. 가장 큰 캔디수를 구한다.
    let max = candies.iter().max().map_or(0, |&x| x);

    // 2. (현재 캔디수 + 여분 캔디수) >= 가장 큰 캔디수 이면 true, 그렇지 않으면 false.
    candies.iter().map(|x| x + extra_candies >= max).collect()
}
