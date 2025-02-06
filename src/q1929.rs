pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();

    for n in &nums {
        let value: i32 = *n;
        result.push(value);
    }

    for n in &nums {
        let value: i32 = *n;
        result.push(value);
    }

    result
}
