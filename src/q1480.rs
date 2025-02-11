pub fn running_sum(nums: &[i32]) -> Vec<i32> {
    let mut results: Vec<i32> = Vec::new();
    let mut sum = 0;
    for i in nums {
        sum += i;
        results.push(sum);
    }
    results
}
