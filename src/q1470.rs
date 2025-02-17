pub fn shuffle(nums: &[i32], n: usize) -> Vec<i32> {
    (0..n).flat_map(|i| vec![nums[i], nums[i + n]]).collect()
}
