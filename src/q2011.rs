pub fn final_value_after_operations(operations: Vec<&str>) -> i32 {
    let mut result = 0;

    for operation in operations {
        if operation.starts_with('-') || operation.ends_with('-') {
            result -= 1;
        } else {
            result += 1;
        }
    }

    result
}
