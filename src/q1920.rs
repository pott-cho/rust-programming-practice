pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    for i in 0..nums.len() {
        let index = nums[i] as usize;
        result.push(nums[index]);
    }
    result
}
