pub fn create_target_array(nums: &[i32], index: &[usize]) -> Vec<i32> {
    let size = nums.len();
    let mut result = Vec::with_capacity(size);

    for i in 0..size {
        result.insert(index[i], nums[i]);
    }

    result
}
