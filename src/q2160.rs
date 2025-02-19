pub fn minimum_sum(num: i32) -> u32 {
    let mut digits: Vec<u32> = num
        .to_string()
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect();

    digits.sort_unstable();

    digits[0] * 10 + digits[2] + digits[1] * 10 + digits[3]
}
