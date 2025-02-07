pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
    let mut sum = 0;

    for account in accounts {
        // 1. 각 배열의 합을 구한다.
        let mut temp = 0;
        for num in account {
            temp += num;
        }
        // 2. 배열의 합이 가장 큰 값을 찾는다.
        sum = if sum < temp { temp } else { sum };
    }

    sum
}
