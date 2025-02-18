pub fn running_sum(nums: &[i32]) -> Vec<i32> {
    nums.iter()
        .scan(0, |sum, &n| {
            *sum += n;
            Some(*sum)
        })
        .collect()
}
