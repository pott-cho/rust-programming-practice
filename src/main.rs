mod q1108;
mod q1281;
mod q1365;
mod q1431;
mod q1470;
mod q1480;
mod q1512;
mod q1672;
mod q1920;
mod q1929;
mod q2160;

fn main() {
    let result = q1929::get_concatenation(&[1, 2, 1]);
    assert_eq!(result, vec![1, 2, 1, 1, 2, 1]);

    let result = q1920::build_array(&[0, 2, 1, 5, 3, 4]);
    assert_eq!(result, [0, 1, 2, 4, 5, 3]);

    let result = q1480::running_sum(&[1, 2, 3, 4]);
    assert_eq!(result, Vec::from([1, 3, 6, 10]));

    let result = q1672::maximum_wealth(vec![vec![1, 2, 3], vec![3, 2, 1]]);
    assert_eq!(result, 6);

    let result = q1470::shuffle(&[2, 5, 1, 3, 4, 7], 3);
    assert_eq!(result, vec![2, 3, 5, 4, 1, 7]);

    let result = q2160::minimum_sum(2932);
    assert_eq!(result, 52);
    let result = q2160::minimum_sum(4009);
    assert_eq!(result, 13);

    let result = q1512::num_identical_pairs(&[1, 2, 3, 1, 1, 3]);
    assert_eq!(result, 4);
    let result = q1512::num_identical_pairs(&[1, 1, 1, 1]);
    assert_eq!(result, 6);
    let result = q1512::num_identical_pairs(&[1, 2, 3]);
    assert_eq!(result, 0);

    let result = q1431::kids_with_candies(&[2, 3, 5, 1, 3], 3);
    assert_eq!(result, vec![true, true, true, false, true]);

    let result = q1281::subtract_product_and_sum(234);
    assert_eq!(result, 15);

    let result = q1365::smaller_numbers_than_current(&[8, 1, 2, 2, 3]);
    assert_eq!(result, vec![4, 0, 1, 1, 3]);
    let result = q1365::smaller_numbers_than_current(&[7, 7, 7, 7]);
    assert_eq!(result, vec![0, 0, 0, 0]);

    let result = q1108::defang_i_paddr("1.1.1.1");
    assert_eq!(result, String::from("1[.]1[.]1[.]1"));
}
