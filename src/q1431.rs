pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
    let mut results = Vec::new();

    // 1. 가장 큰 캔디수를 구한다.
    let max = candies.iter().max().unwrap();

    // 2. (현재 캔디수 + 여분 캔디수) >= 가장 큰 캔디수 이면 true, 그렇지 않으면 false.
    for c in &candies {
        if c + extra_candies >= *max {
            results.push(true);
        } else {
            results.push(false);
        }
    }

    results
}
