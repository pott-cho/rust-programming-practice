pub fn subtract_product_and_sum(n: i32) -> i32 {
    // 1. 각 자리수의 배열 nums를 구한다.
    let mut nums = Vec::new();

    let mut num = n;
    while num > 0 {
        nums.push(num % 10);
        num /= 10;
    }

    // 2. 모든 자리수의 곱셈합(product) 와 덧셈합(sum) 을 구한다.
    let mut product = 1;
    let mut sum = 0;
    for n in &nums {
        product *= n;
        sum += n;
    }

    // 3. 결과를 출력한다.
    product - sum
}
