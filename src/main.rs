mod q1929;

fn main() {
    let result = q1929::get_concatenation(Vec::from([1, 2, 1]));
    assert_eq!(result, vec![1, 2, 1, 1, 2, 1]);
}
