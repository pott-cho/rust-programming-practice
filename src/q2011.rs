pub fn final_value_after_operations(operations: &[&str]) -> i32 {
    operations.iter().fold(0, |acc, x| {
        if x.starts_with('-') || x.ends_with('-') {
            acc - 1
        } else {
            acc + 1
        }
    })
}
