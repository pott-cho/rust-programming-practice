use std::iter::repeat;

pub fn decompress_rl_elist(nums: &[usize]) -> Vec<usize> {
    nums.chunks(2)
        .flat_map(|chunk| {
            let freq = chunk[0];
            let val = chunk[1];
            repeat(val).take(freq)
        })
        .collect()
}
