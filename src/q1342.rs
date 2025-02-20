pub fn number_of_steps(num: i32) -> u32 {
    if num == 0 {
        return 0;
    }

    let even = 32 - num.leading_zeros(); // 이진수의 크기 == 나누기 2를 해야하는 횟수
    let odd = num.count_ones(); // 1의 개수 == 빼기 1을 해야하는 횟수

    even + odd - 1 // 마지막 1처리
}
