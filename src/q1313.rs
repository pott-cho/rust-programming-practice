use std::iter::repeat;

pub fn decompress_rl_elist(nums: &[usize]) -> Vec<usize> {
    nums.chunks(2)
        .filter_map(|chunk| match chunk {
            [freq, val] => Some((freq, val)),
            _ => None,
        })
        .flat_map(|(&freq, &val)| repeat(val).take(freq))
        .collect()
}
