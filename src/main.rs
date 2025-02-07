mod q1470;
mod q1480;
mod q1672;
mod q1920;
mod q1929;

fn main() {
    let result = q1929::get_concatenation(&[1, 2, 1]);
    assert_eq!(result, vec![1, 2, 1, 1, 2, 1]);

    let result = q1920::build_array(&[0, 2, 1, 5, 3, 4]);
    assert_eq!(result, [0, 1, 2, 4, 5, 3]);

    let result = q1480::running_sum(&[1, 2, 3, 4]);
    assert_eq!(result, Vec::from([1, 3, 6, 10]));

    let result = q1672::maximum_wealth(vec![vec![1, 2, 3], vec![3, 2, 1]]);
    assert_eq!(result, 6);

    let result = q1470::shuffle(vec![2, 5, 1, 3, 4, 7], 3);
    assert_eq!(result, vec![2, 3, 5, 4, 1, 7]);
}
