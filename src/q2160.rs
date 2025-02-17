pub fn minimum_sum(num: i32) -> i32 {
    let mut num = num;
    let mut digits: Vec<i32> = (0..4)
        .map(|_| {
            let digit = num % 10;
            num /= 10;
            digit
        })
        .collect();

    digits.sort_unstable();

    digits[0] * 10 + digits[2] + digits[1] * 10 + digits[3]
}
