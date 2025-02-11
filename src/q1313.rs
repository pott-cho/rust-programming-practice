pub fn decompress_rl_elist(nums: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();

    for i in (0..nums.len()).step_by(2) {
        let freq = nums[i];
        let val = nums[i + 1];

        let mut f = 0;
        while f < freq {
            result.push(val);
            f += 1;
        }
    }

    result
}
