pub fn number_of_steps(num: i32) -> i32 {
    let mut number = num;
    let mut cnt = 0;
    while number > 0 {
        if number % 2 == 0 {
            number /= 2;
        } else {
            number -= 1;
        }
        cnt += 1;
    }
    cnt
}
