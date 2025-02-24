pub fn subtract_product_and_sum(n: i32) -> u32 {
    // 1. 각 자리수의 배열 nums를 구한다.
    let arr: Vec<u32> = n
        .to_string()
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect();

    // 2. 모든 자리수의 곱셈합(product) 와 덧셈합(sum) 을 구한다.
    let (product, sum) = arr
        .iter()
        .fold((1, 0), |(p, s), digit| (p * digit, s + digit));

    product - sum
}
