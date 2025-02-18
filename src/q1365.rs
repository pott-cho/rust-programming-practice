pub fn smaller_numbers_than_current(nums: &[usize]) -> Vec<usize> {
    let result = nums
        .iter()
        .map(|x| nums.iter().filter(|y| **y < *x).count())
        .collect();

    result
}
