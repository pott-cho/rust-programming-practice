pub fn minimum_sum(num: i32) -> i32 {
    let mut n = num;

    // 1. num의 각 자리 수로 배열을 만든다.
    let mut nums = Vec::new();
    for _i in 0..4 {
        nums.push(n % 10);
        n /= 10;
    }

    // 2. 배열을 정렬한다.
    nums.sort_unstable();

    // 3. 정렬한 배열을 이용해서 만들 수 있는 가장 작은 수 2개를 만든다. (0, 2) (1, 3)
    let first = if nums[0] > 0 {
        nums[0] * 10 + nums[2]
    } else {
        nums[2]
    };
    let second = if nums[1] > 0 {
        nums[1] * 10 + nums[3]
    } else {
        nums[3]
    };

    first + second
}
