pub fn get_decimal_value(arr: &[i32]) -> i32 {
    let mut mul = -1;
    arr.iter().rev().fold(0, |acc, x| {
        mul += 1;
        acc + (*x << mul)
    })
}
