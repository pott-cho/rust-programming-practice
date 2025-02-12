pub fn smaller_numbers_than_current(nums: &[usize]) -> Vec<usize> {
    //1. nums 를 정렬한다.
    let mut sorted_nums = nums.to_vec();
    sorted_nums.sort_unstable();

    // 2. 모든 자릿수에 대해 해당 자릿수보다 작은 수의 개수를 저장하는 board를 만든다.
    let mut board = [0; 101];

    // 3. 정렬된 nums 에서 이전 숫자가 현재숫자보다 작으면 이전 인덱스 번호가 (현재숫자보다 작은 수의
    //    개수 - 1) 이 된다.
    for i in 1..sorted_nums.len() {
        if sorted_nums[i - 1] < sorted_nums[i] {
            board[sorted_nums[i]] = i;
        } else {
            board[sorted_nums[i]] = board[sorted_nums[i - 1]];
        }
    }

    // 4. 최종적으로 정렬전의 nums 를 순회하면서 각 숫자보다 작은수의 개수를 board[숫자]를 통해 가져온다.
    let mut results = Vec::with_capacity(nums.len());
    for &n in nums {
        let a = board[n];
        results.push(a);
    }

    results
}
