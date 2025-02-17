pub fn build_array(nums: &[usize]) -> Vec<usize> {
    nums.iter().filter_map(|&i| nums.get(i)).copied().collect()
}
