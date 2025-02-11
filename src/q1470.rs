pub fn shuffle(nums: &[i32], n: usize) -> Vec<i32> {
    let mut result = Vec::with_capacity(nums.len());

    for i in 0..n {
        result.push(nums[i]);
        result.push(nums[i + n]);
    }

    result
}
