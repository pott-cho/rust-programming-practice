pub fn build_array(nums: &[usize]) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    for i in 0..nums.len() {
        let index = nums[i];
        result.push(nums[index].try_into().unwrap());
    }
    result
}
