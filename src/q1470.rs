pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
    let mut result = Vec::with_capacity(nums.len());

    let half_size: usize = n as usize;
    for i in 0..half_size {
        result.push(nums[i]);
        result.push(nums[i + half_size]);
    }
    
    result
}
